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


	
	

		



## reference
- [rust-raspberrypi-OS-tutorials's chap 6 README.md](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/06_drivers_gpio_uart) 
- [rust lifetime](rust_lifetime.md)

