---
- Python >= 3.8

기본 설치는 pure-stdlib이며 SSO 유틸리티를 제공합니다. 생성된 API 클라이언트(`DefaultApi`/`PublicApi`/`ModerationApi`)는 `client` 추가 기능이 필요하며, 이는 `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`, `pydantic >= 2.0.0`, `typing-extensions >= 4.0.0`을 가져옵니다:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```
---