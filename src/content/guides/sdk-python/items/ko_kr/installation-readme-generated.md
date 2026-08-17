### GitHub에서 설치

릴리스 태그에서 직접 설치합니다 (권장, 완전 재현 가능):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

빌드가 결정론적이도록 브랜치가 아니라 태그를 고정하세요. 같은 형식이 `requirements.txt`에서도 작동합니다:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

각 태그된 [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases)에는 바이너리 아티팩트를 직접 설치하고 싶을 경우를 위한 빌드된 wheel도 첨부되어 있습니다.

### 라이브러리 내용

이 라이브러리는 두 개의 모듈을 포함합니다: 생성된 API 클라이언트와 API 사용을 더 쉽게 해주는 수동으로 작성된 유틸리티를 포함한 핵심 Python 라이브러리이며, 여기에는 SSO 지원도 포함됩니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [핵심 라이브러리 문서, SSO 예제 포함](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 가지 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 브라우저/모바일 기기 등에서 인증 없이 직접 호출할 수 있는 메서드를 포함합니다. `ModerationApi`는 실시간 및 빠른 검토를 위한 광범위한 API 모음을 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받아 SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.