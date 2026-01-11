### PyPI

```bash
pip install fastcomments
```

### 라이브러리 구성

이 라이브러리는 두 개의 모듈을 포함합니다: 생성된 API 클라이언트와, SSO 지원을 포함하여 API 사용을 더 쉽게 해주는 수작업 유틸리티가 들어있는 코어 Python 라이브러리입니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [코어 라이브러리 문서, SSO 예제 포함](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `DefaultApi`와 `PublicApi`라는 두 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고 있으며, `PublicApi`는 인증 없이 브라우저/모바일 디바이스 등에서 직접 호출할 수 있는 API 호출을 포함합니다.