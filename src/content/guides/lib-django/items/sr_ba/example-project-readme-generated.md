A runnable showcase lives in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): a left-rail + main-stage app with a page per widget and a **sign-in page listing pre-seeded demo users**.  
Prijavite se kao bilo koji od njih i widgeti za komentare i live‑chat autentifikuju taj identitet putem **Secure SSO**.  
Iz tog direktorija:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Bez API tajne vraća se na javnog `demo` tenant (anoniman).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) je Playwright e2e koji učitava stranicu u pravom pretraživaču i objavljuje komentar kao Secure-SSO korisnik.