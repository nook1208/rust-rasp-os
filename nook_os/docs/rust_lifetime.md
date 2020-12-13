# Rust Lifetime 정리

러스트의 모든 참조자는 라이프타임(liftetime)을 갖으며 이는 해당 참조자가 유효한 스코프를 의미한다.  
대부분은 라이프타임이 암묵적이고 추론되지만 라이프타임을 명시해야만 하는 상황이 존재하므로 라이프타임을 알아보자.

## 라이프타임의 주요 목적
라이프타임의 주목적은 댕글링 참조자(dangling reference)를 방지하는 것인데 이는 우리가 참조하기로 의도한 데이터가 아닌 다른 데이터를 참조하는 원인이 된다. 아래 예시를 살펴보자.
```rust
{ 
    let r; 
    { 
        let x = 5; 
        r = &x; 
    } 
    println!("r: {}", r); 
}
```

```rust
error: `x` does not live long enough 
   | 
6  |         r = &x; 
   |              - borrow occurs here 
7  |     } 
   |     ^ `x` dropped here while still borrowed 
... 
10 | } 
   | - borrowed value needs to live until here
```

위의 에러는 `r` 이 참조하고있는 `x` 의 라이프타임이 내부 스코프가 끝나면서 드랍되는데,   
이런 상황에서 `r` 이 사용된다면 그 동작이 어떤 것이든 정확히 동작하지 않을 것이다.   
러스트는 라이프타임을 통해 이런 문제를 미연에 방지해줄 수 있다.

### 빌림 검사기(Borrow checker)
빌림 검사기(Borrow checker) 라고 불리는 컴파일러의 부분이 모든 빌림이 유효한지를  결정하기 위해 스코프를 비교한다.  
위 예제의 경우 빌림 검사기가 어떻게 동작하는지 확인해보자. 
```rust
    let r;         // -------+-- 'a 
                   //        | 
    {              //        | 
        let x = 5; // -+-----+-- 'b 
        r = &x;    //  |     | 
    }              // -+     | 
                   //        | 
    println!("r: {}", r); // | 
                   //        | 
                   // -------+ 
}
```
`r`, `x` 의 라이프타임을 `'a` `'b` 라고 할 때  `'b` 라이프타임이 `'a` 라이프타임에 비해 작기 때문에 러스트 컴파일러는 이 프로그램을 거부한다.  
참조자의 주체`'a`가 참조자`'b`만큼 오래 살지 못하기 때문이다.  

## 함수에서의 제네릭 라이프타임
두 스트링 슬라이스 중에서 긴 쪽을 반환하는 함수를 작성하여 라이프타임 명시가 필요한 상황을 확인해보자.
```rust
fn longest(x: &str, y: &str) -> &str { 
    if x.len() > y.len() { 
        x 
    } else { 
        y 
    } 
}

fn main() { 
    let string1 = String::from("abcd"); 
    let string2 = "xyz"; 
    let result = longest(string1.as_str(), string2); 
    println!("The longest string is {}", result); 
}
```

longest 함수를 구현하면 `The longest string is abcd` 를 출력해야 한다.  허나 이 프로그램은 동작하지 않을 것이다.
```rust
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`
```
에러 메시지는 반환할 참조자의 라이프타임을 명시하라고 한다.  그 이유는 반환할 참조자가 어떤 참조자인지 컴파일러가 알 수 없기 때문이다.

##  함수 시그니처 내의 라이프타임 명시
#### 라이프타임 명시 문법
```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
라이프타임 파라미터의 이름은 어퍼스트로피' 로 시작하며 라이프타임 파라미터의 이름은 보통 모두 소문자이다.  
또한 라이프타임 파라미터 명시는 참조자의 & 뒤에 오며 공백 문자가 라이프타임 명시와 참조자의 타입을 구분해준다.  

그럼 이제 위에서 보았던 longest 함수에 라이프타임을 명시하여 프로그램이 정상동작하게 해보자.
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
이 함수에서 모든 참조자에 동일한 라이프타임 `'a` 를 선언하였는데, 이는 파라미터로 들어오는 `x`, `y` 모두  
동일한 라이프타임을 갖고있어야 한다는 것이고 반환되는 참조자 또한 적어도 `'a` 만큼 살아있어야 한다는 것이다.  
빌림 검사기는 이 내용을 기반으로 해당 함수에 입력될 참조자들이 적합한 라이프타임을 갖는지 그렇지 않는지 검사하여 알려줄 것이다.

서로 다른 라이프타임을 갖는 참조자를 longest 함수의 파라미터로 사용하는 예제이며 이 프로그램은 잘 동작한다.
```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

다음으로, result의 참조자의 라이프타임이 두 인자들의 라이프타임보다 작아야 함을 보여줄 예제이다.
```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

```rust
error: `string2` does not live long enough
   |
6  |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
```

이 에러는 result가 println!에서 유효하기 위해서는. string2가 외부 스코프의 끝까지 유효할 필요가 있음을 말해준다.  
러스트는 이를 알고 있는데, 그 이유는 우리가 함수의 파라미터들과 반환 값에 대해 동일한 라이프타임 파라미터 `'a`를 명시했기 때문이다.

라이프타임 파라미터를 특정하는 정확한 방법은 그 함수가 어떤 일을 하느냐에 따라 달려있다.  
예를들어 longest 함수의 구현을 항상 첫 번째 인자를 반환하도록 바꾼다면 y 파라미터에 대한 라이프타임은 특정할 필요가 없다.
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

## 구조체 정의 상에서의 라이프타임 명시
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```
이 구조체는 스트링 슬라이스를 담을 수 있는 part라는 하나의 필드를 갖고 있는데, 이것이 참조자이다.  
제네릭 데이터 타입과 마찬가지로, 제네릭 라이프타임 파라미터의 이름을 구조체의 이름 뒤편에 꺾쇠괄호 안에다 선언하여 구조체 정의의 본체 내에서 이 라이프타임 파라미터를 이용할 수 있도록 해야한다.  


## 라이프타임 생략
러스트 팀은 러스트 컴파일러 코드 내에 이 패턴들을 프로그래밍하여 이러한 상황 내에서는 프로그래머가 명시적으로 라이프타임 명시를 추가하도록 강제하지 않고 빌림 검사기가 라이프타임을 추론할 수 있도록 하였다.  
그 결과로, 굳이 명시하지 않아도 되는 라이프타임들을 일일이 작성하는 일은 상당히 많이 사라졌으며,  
추론 가능한 조건들이 더 많이 생길 수록 라이프타임을 생략할 수 있는 경우가 많아질 예정이라고 한다.

그럼 라이프타임 생략 규칙에 대해 알아보자.
1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 갖는다.   
`fn foo<'a>(x: &'a i32)`,   
`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`

2. 만일 정확히 딱 하나의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입된다.  
`fn foo<'a>(x: &'a i32) -> &'a i32` .

3. 만일 여러 개의 입력 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 &self  혹은 &mut self 라고 한다면, self 의 라이프타임이 모든 출력 라이프타임 파라미터에 대입된다.

위의 규칙들에 포함되지 않는 라이프타임은 모두 명시가 필요한 라이프타임이며 빌드에러를 통해 어떤 참조자에 라이프타임 명시가 필요한 지 알 수 있을 것이다.

## 메소드 정의 내에서의 라이프타임 명시
구조체 필드를 위한 라이프타임 이름은 언제나 impl 키워드 뒤에 선언되어야 하며, 그러고 나서 구조체의 이름 뒤에 사용되어야 하는데, 이 라이프타임들은 구조체 타입의 일부이기 때문이다.
```rust
impl<'a> ImportantExcerpt<'a> {
    // 첫 번째 생략 규칙으로 인해 self 로의 참조자의 라이프타임을 명시할 필요가 없음.
    fn level(&self) -> i32 { 
        3
    }

    // 첫 번째 생략 규칙으로 인해 두 파라미터의 라이프타임이 생략됬으며
    // 세 번째 생략 규칙으로 인해 반환 타입은 &self 의 라이프타임을 얻고, 모든 라이프타임들이 추론되었음.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement); 
        self.part
    }
}
```

## 정적 라이프타임(Static lifetime)
`'static`라이프타임은 프로그램의 전체 생애주기를 가리킨다.   
모든 스트링 리터럴은 `'static` 라이프타임을 가지고 있는데, 아래와 같이 명시하는 쪽을 선택할 수 있다.
```rust
let s: &'static str = "I have a static lifetime.";
```
가끔 `'static` 라이프타임을 이용하라는 제안을 볼 수 도 있는데,   
정말 해당 변수가 프로그램이 끝날 때 까지 라이프타임이 필요한 것인지 따져봐야만 한다.

## 제네릭 타입 파라미터, 트레잇 바운드, 라이프타임을 함께 써보기
제네릭 타입 파라미터, 트레잇 바운드, 라이프타임 모두를 적용 했을 때 아래와 같이 코드 작성이 가능하다.
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```




## reference
https://rinthel.github.io/rust-lang-book-ko/ch10-03-lifetime-syntax.html

