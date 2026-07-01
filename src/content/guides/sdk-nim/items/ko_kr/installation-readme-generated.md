### Nimble 사용

```bash
nimble install fastcomments
```

### 소스에서 빌드

```bash
nimble build
```

### 라이브러리 내용

이 라이브러리는 생성된 API 클라이언트와 API 사용을 쉽게 해주는 SSO 유틸리티를 포함하고 있습니다.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `api_default`, `api_public`, `api_moderation` 세 가지 API 모듈이 있습니다. `api_default`는 API 키가 필요한 메서드를 포함하고, `api_public`은 인증 없이 브라우저/모바일 디바이스 등에서 직접 호출할 수 있는 API 호출을 포함합니다. `api_moderation` 모듈은 관리자 대시보드를 위한 메서드를 포함합니다.

`api_moderation` 모듈은 실시간 및 빠른 검토 API의 포괄적인 스위트를 제공합니다. 모든 `api_moderation` 메서드는 `sso` 매개변수를 받아들이며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.