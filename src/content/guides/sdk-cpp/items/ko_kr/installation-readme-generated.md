### Install Dependencies

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Building from Source

```bash
mkdir build
cd build
cmake ..
make
```

### Installing

```bash
sudo make install
```

### Library Contents

이 라이브러리에는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티가 포함되어 있습니다.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 개의 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 브라우저/모바일 장치 등에서 인증 없이 직접 호출할 수 있는 메서드를 포함합니다. `ModerationApi`는 실시간 및 빠른 모더레이션 API를 광범위하게 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받아들이며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.