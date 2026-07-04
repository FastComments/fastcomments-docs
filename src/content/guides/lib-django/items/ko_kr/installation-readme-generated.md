---
릴리스 태그에서 설치하기 (이 프로젝트는 PyPI가 아니라 git 태그를 통해 배포됩니다):

```bash
pip install "git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

서버 측 REST 접근(`admin()` / `public_api()` 도우미)을 위해 `api` 추가 기능을 추가하면 SDK에서 생성된 클라이언트를 가져옵니다:

```bash
pip install "fastcomments-django[api] @ git+https://github.com/fastcomments/fastcomments-django.git@v0.1.0"
```

앱을 `INSTALLED_APPS`에 추가합니다:

```python
INSTALLED_APPS = [
    # ...
    "fastcomments_django",
]
```
---