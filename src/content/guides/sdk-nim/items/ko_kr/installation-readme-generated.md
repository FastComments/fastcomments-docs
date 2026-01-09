### Nimble 사용하기

```bash
nimble install fastcomments
```

### 소스에서 빌드하기

```bash
nimble build
```

### 라이브러리 구성

이 라이브러리는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티를 포함합니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `api_default`와 `api_public`의 두 가지 API 모듈이 있습니다. `api_default`는 API 키가 필요한 메서드를 포함하고, `api_public`은 API 호출을 포함
인증 없이 브라우저/모바일 기기 등에서 직접 호출할 수 있습니다.