# Atomic Opertion
Atomic operation 은 공유된 자원에 대해서 멀티 스레드 혹은 멀티 프로세서로 부터 조작(operation)이 발생할 때 그 조작 중간에 끼어들거나 분할되지 않고 한번에 이뤄지는 operation 을 말한다.

## Exclusive monitors
exclusive monitor 란 `open`, `exclusive` 라는 두 state 를 가지는 simple state machine 이다.  
아키텍쳐는 프로세서간의 동기화를 위해 local, global 두 종류의 exclusive monitor 를 갖고 있다.

![image](https://user-images.githubusercontent.com/50063698/111892987-13872d00-8a43-11eb-8177-fd61487c6fc1.png)
### local monitor
- local monitor는 코어 내의 존재한다.
- non-sharable 영역에 대해 접근되는 동일 코어의 서로 다른 task 들간의 접근을 관측(monitoring)한다.

### global monitor
- global monitor 는 AXI 버스와 Memory 인터페이스 중간에 연결되어있다.
- sharable 영역에 대해 접근되는 서로 다른 master 들간의 접근을 관측한다.

## Instructions for atomicity
armv8 은 서로 다른 task 혹은 master 에 의해 공유되는 메모리영역에 대해 exclusive memory access 를 제공하는 instruction 들이 있다.

### ldxr
`LDXR <Xt>, [Xn]`
- exclusive monitor 에 `exclusive` state 를 설정
- Xn 의 address 로 부터 값을 Xt 에 load

### stxr
`STXR <Ws>, <Xt>, [Xn]`
- Xn 주소 영역에 접근 시 `exclusive` state 일 때 Xt 의 값을 Xn 주소에 store
- <Ws> 값이 0 이면 store 성공, 1 이면 실패
- store 성공 시 Xn 주소 영역은 `open` state 로 변경됨

![image](https://user-images.githubusercontent.com/50063698/111892923-ea669c80-8a42-11eb-8e69-12fd1853b3f3.png)

## Load-linked/store-conditional(LL/SC)
LL/SC 는 아키텍처에서 사용하는 atomic 연산 구현방식 중 하나이며 멀티스레드 상황에서 동기화를 얻기위한 load-store 로 이뤄진 한 쌍의 명령어이다.  
Load-link 는 메모리 영역의 현재 값을 반환하는 반면, store-conditional 은 동일 메모리 영역에 load-link 이후로 업데이트가 발생하지 않은 경우에만 
해당 영역에 새로운 값을 저장한다.  
이는 lock 없이 atomic 하게 공유 자원을 변경할 수 있는 방식이다.

### LL/SC e.g: xchg
```assembly
xchg:
    prfm pstl1strm [x1]
1:  ldxr x0, [x1]
    stxr w3, x2, [x1]
    cbnz w3, 1b
    
    ret
```
- x1: store 대상 주소
- x2: store 할 값  

x1 주소 영역의 값을 x2 의 값으로 update 후 기존 x1 주소에 저장되었던 값을 반환하는 xchg 함수의 예제이다.  
stxr 가 실패할 시에 cbnz 명령을 통해 다시 `1:` 로 분기하여 ldxr 를 다시 시도한다.

### LL/SC e.g: cmpxchg
```assembly
cmpxchg:
    prfm pstl1strm [x1]
1:  ldxr x9, [x1]
    eor x8, x9, x0
    cbnz x8, 2f
    stxr w8, x2, [x1]
    cbnz w8, 1b
2:
    mov x0, x9
    ret
```
- x0: cmpxchg 함수 수행 전, x1 주소 영역에 저장되어 있던 값 (old_val)
- x1: store  주소
- x2: store 할 값  

xchg 와 동일하나, store 시도 전, x1 주소영역에 저장된 값이 old_val 와 일치하는지 검사하는 부분이 추가되었다.



## references
- https://en.wikipedia.org/wiki/Load-link/store-conditional  
- https://developer.arm.com/documentation/dht0008/a/arm-synchronization-primitives/exclusive-accesses/exclusive-monitors  
- http://jake.dothome.co.kr/atomic/  
- http://jake.dothome.co.kr/exclusive-loads-and-store/  
- DDI0487E_a_armv8_arm.pdf
- DDI_0596_ARM_a64_instruction_set_architecture.pdf
