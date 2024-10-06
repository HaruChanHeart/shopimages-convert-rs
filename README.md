# ShopImage Converter for Overwatch
오버워치 2의 상점 이미지 파일을 PNG 파일로 변환해 주는 프로그램입니다

### 참고용: 상점 이미지 파일 포맷 정보
* `0x00 - 0x03` 헤더 및 매직 넘버
* `0x04 - 0x05` DDS 포맷 Y값
* `0x06 - 0x07` DDS 포맷 X값
* `0x08 - 0x0B` 파일의 주소 크기