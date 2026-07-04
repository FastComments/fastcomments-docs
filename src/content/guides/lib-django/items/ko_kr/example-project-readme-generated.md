A runnable showcase lives in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): a left-rail + main-stage app with a page per widget and a **sign-in page listing pre-seeded demo users**.  
Sign in as any of them and the comment and live-chat widgets authenticate that identity via **Secure SSO**.  
From that directory:

```bash
python manage.py migrate
# 자체 테넌트를 사용하여 Secure SSO가 작동하는 것을 확인하세요 (API 비밀키가 이를 활성화합니다):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Without an API secret it falls back to the public `demo` tenant (anonymous).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) is a Playwright e2e that loads the page in a real browser and posts a comment as the Secure-SSO user.