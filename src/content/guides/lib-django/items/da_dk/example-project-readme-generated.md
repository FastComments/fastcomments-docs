A runnable showcase lives in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): a left-rail + main-stage
app with a page per widget and a **sign-in page listing pre-seeded demo users**.  
Log ind som en af dem, og kommentar‑ og live‑chat‑widgetsene autentificerer den identitet via **Secure SSO**. Fra den mappe:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Uden en API secret falder den tilbage til den offentlige `demo` tenant (anonymous).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) er en Playwright e2e
der indlæser siden i en rigtig browser og poster en kommentar som Secure‑SSO‑brugeren.