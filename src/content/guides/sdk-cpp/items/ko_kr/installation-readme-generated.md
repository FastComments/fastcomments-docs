### 의존성 설치

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

### 라이브러리 구성

이 라이브러리는 생성된 API 클라이언트와 API 작업을 더 쉽게 만들어 주는 SSO 유틸리티를 포함하고 있습니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 공개 API vs 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 가지 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하며, `PublicApi`는 인증 없이 브라우저/모바일 기기 등에서 직접 호출할 수 있는 메서드를 포함합니다. `ModerationApi`는 실시간 빠른 검토(모더레이션)를 위한 폭넓은 API 모음을 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받으며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.