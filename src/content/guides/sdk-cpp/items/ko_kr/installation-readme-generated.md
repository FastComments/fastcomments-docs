### 종속성 설치

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### 소스에서 빌드하기

```bash
mkdir build
cd build
cmake ..
make
```

### 설치하기

```bash
sudo make install
```

### 라이브러리 내용

이 라이브러리에는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티가 포함되어 있습니다.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 공개 API와 보호된 API

API 클라이언트에는 `DefaultAPI`와 `PublicAPI`의 두 클래스가 있습니다. `DefaultAPI`는 API 키가 필요한 메서드를 포함하고, `PublicAPI`는 브라우저/모바일 기기 등에서 인증 없이 직접 호출할 수 있는 API 호출을 포함합니다.