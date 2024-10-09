![Header Image](./docs/image.png)

# ShopImage Converter for Overwatch
오버워치 2의 상점 이미지 파일을 PNG 파일로 변환해 주는 프로그램입니다

## 사용 순서
1. 프로그램을 실행하면 자동으로 `input` 디렉터리와 `output` 디렉터리가 만들어집니다
2. **게임 실행 후 상점**으로 들어갑니다. 그러면 최근 상점 이미지들이 저장됩니다
3. `%LocalAppData%\Blizzard Entertainment\Overwatch\ShopImages`에 있는 파일들을 `input` 디렉터리 안으로 복제합니다
4. 프로그램 실행하면 이미지 파일들을 PNG 파일로 변환하며, `output` 파일에 저장합니다

### 참고용: 상점 이미지 파일 포맷 정보
* `0x00 - 0x03` 헤더 및 매직 넘버
* `0x04 - 0x05` DDS 포맷 Y값
* `0x06 - 0x07` DDS 포맷 X값
* `0x08 - 0x0B` 파일의 주소 크기