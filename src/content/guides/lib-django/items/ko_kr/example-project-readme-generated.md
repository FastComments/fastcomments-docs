A runnable showcase lives in [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): a left-rail + main-stage app with a page per widget and a **sign-in page listing pre-seeded demo users**.  
`example/`에 실행 가능한 데모가 있습니다: 위젯당 페이지가 있는 왼쪽 레일 + 메인 스테이지 앱이며 **미리 시드된 데모 사용자 목록이 있는 로그인 페이지**가 포함됩니다.

Sign in as any of them and the comment and live‑chat widgets authenticate that identity via **Secure SSO**.  
그 중 하나로 로그인하면 댓글 및 실시간 채팅 위젯이 **Secure SSO**를 통해 해당 신원을 인증합니다.

From that directory:  
해당 디렉터리에서:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

```bash
python manage.py migrate
# 자체 테넌트를 사용하여 Secure SSO 작동을 확인하세요 (API 비밀키가 이를 활성화합니다):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Without an API secret it falls back to the public `demo` tenant (anonymous).  
API 비밀키가 없으면 공개 `demo` 테넌트(익명)로 대체됩니다.

[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) is a Playwright e2e that loads the page in a real browser and posts a comment as the Secure‑SSO user.  
`example/browser_smoke.py`는 실제 브라우저에서 페이지를 로드하고 Secure‑SSO 사용자로서 댓글을 게시하는 Playwright e2e 테스트입니다.