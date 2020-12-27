## tl;dr
- 첫 번째 실제 디바이스 드라이버를 추가하기 위한 인프라 구축  
- 마법과도 같던 QEMU 콘솔을 버리고 진짜 `UART` 를 사용함

## Notable additions
- 처음으로 실제 하드웨어에서 프로그램을 실행시킬 것이다. 
	- 그러므로 빌드는 RPi 3, RPi 4 에 따라 달라질 것이며 기존 repo의 코드와는 다르게 default 로 RPi 4 로 빌드할 예정이다.
	- RPi3 으로 빌드할 경우 `BSP=rpi3` 를 명령어의 앞에 추가해야 한다 :
		- `BSP=rpi3 make` 
		- `BSP=rpi3 make doc` 
	- QEMU 는 안타깝게도 RPi4 지원이 안 되므로 `BSP=rpi3 make qemu`  명령만 동작할 것이다.
	- [RPi4](https://smartstore.naver.com/icfactory/products/4750528335?site_preference=device&NaPm=ct%3Dkimj9wd4%7Cci%3Dshopn%7Ctr%3Dslsl_myz%7Chk%3Dce88f6a9325b5a9045745d7fa486d31c27351729%7Ctrx%3Dundefined) 와 [USB serial debug cable](https://smartstore.naver.com/makerspace/products/2189915982?site_preference=device&NaPm=ct%3Dkimj9xse%7Cci%3Dshopn%7Ctr%3Dslsl_myz%7Chk%3D1ed52ed167d3b3ed39576062b02289f66057c438%7Ctrx%3Dundefined) 을 구매하자. RPi4 을 선택한 건 그냥 RPi3 보다 최신이니까..
- `driver::interface::DeviceDriver`  트레잇은 `BSP` 커널 단에서의 driver 구현의 추상화를 위해 추가되었다.

## Base knowledge for this chapter
- [GPIO](https://ko.wikipedia.org/wiki/GPIO)  
다용도 입출력(general-purpose input/output, GPIO)은 입력이나 출력을 포함한 동작이 런타임 시에 사용자에 의해 제어될 수 있는, 집적 회로나 전기 회로 기판의 디지털 신호 핀이다.  
GPIO는 특정한 목적이 미리 정의되지 않으며 기본적으로는 사용되지 않는다. GPIO는 어셈블리 레벨의 회로망 설계자(집적 회로 GPIO의 경우에는 회로 기판 설계자, 기판 레벨 GPIO의 경우에는 시스템 통합자, S/I)에 의해 구현되어 있으며 사용 시에는 GPIO의 목적과 동작이 정의된다.  
원론적인 내용은 이렇고, 코드 상으로 어떻게 GPIO 접근이 구현되는지 파악하면 더 이해가 될 것 같다.

- [UART](https://m.blog.naver.com/iintuition_/220585614002)  
병렬 형태의 데이터를 직렬 형태의 데이터로 바꾸어서 직렬 통신을 할 수 있게 해주는 하드웨어의 일종이다.  
즉, PC나 MCU(Micro Control Unit) 내에서는 데이터가 병렬 형태로 움직이는데, 이런 데이터들을 다른 기기와 직렬방식으로 통신할 수 있도록 직렬 데이터로 바꾸어주는 눈에 보이는 집적회로이다.  
[RPi4 에서 사용되는 UART 는 PL011](https://wikidocs.net/7974) 로, 광범위하게 호환 가능한 UART 라고 한다.

- [BCM2xxx](https://wikidocs.net/42377)  
라즈베리 파이 모델에서 사용되는 브로드컴 칩의 네이밍이며 RPi4 에서는 BCM2711 을 사용한다.  

- [MMIO](https://ko.wikipedia.org/wiki/%EB%A9%94%EB%AA%A8%EB%A6%AC_%EB%A7%B5_%EC%9E%85%EC%B6%9C%EB%A0%A5)  
메모리 맵 입출력(영어: Memory-mapped I/O, MMIO)는 마이크로프로세서(CPU)가 입출력 장치를 액세스할 때, 입출력과 메모리의 주소 공간을 분리하지 않고 하나의 메모리 공간에 취급하여 배치하는 방식이다.
따라서 전체 메모리의 주소공간에 입출력 장치의 메모리나 레지스터를 메모리로 취급하여 전체 메모리의 일부분으로 특정영역에 할당하여 배치하는 방식이다.
입출력 장치의 메모리 주소가 나뉘어 있지 않기 때문에 액세스할 때는 메모리와 같은 주소공간이므로 같은 기계어 코드로 수행한다.

## Code analysis
### bsp::device_driver
- [PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)  
사용되지 않는 데이터에 대한 컴파일러의 불평을 잠재울 수 있는 일종의 "더미" 데이터이다.

- [Self](https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self)  
Self 는 현재 object 의 type 이다.


### 'register' crate added
GPIO 관련 코드에서 [register crate](https://github.com/rust-embedded/register-rs) 를 사용하여 각종 레지스터들을 정의하는데   
여기서 사용되는 API 는 [Tock Register Interface](https://github.com/tock/tock/tree/master/libraries/tock-register-interface) 이며 사용방법과 API의 대략적인 정리가 필요하다.  

Analyzing...


	
	

		



## references
- [rust-raspberrypi-OS-tutorials's chap 6 README.md](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/06_drivers_gpio_uart) 
- [rust lifetime](rust_lifetime.md)

