### Install from GitHub

릴리즈 태그에서 직접 설치합니다 (권장, 완전 재현 가능).

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

빌드가 결정론적이도록 브랜치가 아니라 태그를 고정합니다. 같은 형태가 `requirements.txt`에서도 동작합니다:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

각 태그된 [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases)에도 바이너리 아티팩트를 직접 설치하고 싶을 경우 사용할 수 있는 wheel 파일이 함께 제공됩니다.

### Library Contents

이 라이브러리는 두 개의 모듈을 포함합니다: 자동 생성된 API 클라이언트와 API 사용을 더 쉽게 해주는 수동 구현 유틸리티를 포함하고 있는 핵심 Python 라이브러리(SSO 지원 포함).

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 가지 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 인증 없이 브라우저/모바일 디바이스 등에서 직접 호출할 수 있는 메서드를 포함합니다. `ModerationApi`는 실시간 및 빠른 검토를 위한 방대한 모듈의 검토 API를 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받아 SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.