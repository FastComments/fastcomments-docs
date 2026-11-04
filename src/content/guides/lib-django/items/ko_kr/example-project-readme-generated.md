실행 가능한 쇼케이스가 [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example)에 있습니다: 좌측 레일 + 메인 스테이지
구성의 앱으로, 위젯별 페이지와 **사전 시드된 데모 사용자 목록을 보여주는 로그인 페이지**가 있습니다.
이 사용자 중 누구로든 로그인하면 댓글 및 라이브 채팅 위젯이 **Secure SSO**를 통해 해당 신원을 인증합니다.
해당 디렉터리에서:

```bash
python manage.py migrate
# Secure SSO가 작동하는 모습을 보려면 자신의 테넌트를 사용하세요 (API 시크릿이 이를 활성화합니다):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

API 시크릿이 없으면 공개 `demo` 테넌트(익명)로 폴백합니다.
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py)는 실제 브라우저에서 페이지를 로드하고
Secure-SSO 사용자로 댓글을 게시하는 Playwright e2e입니다.