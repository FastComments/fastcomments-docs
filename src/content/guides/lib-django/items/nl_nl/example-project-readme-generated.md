Een uitvoerbare showcase bevindt zich in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): een left-rail + main-stage app met een pagina per widget en een **inlogpagina die vooraf ingestelde demo‑gebruikers vermeldt**.  
Log in als een van hen en de commentaar‑ en live‑chat‑widgets authenticeren die identiteit via **Secure SSO**. Van die map:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Zonder een API‑secret valt het terug naar de publieke `demo` tenant (anoniem).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) is een Playwright‑e2e die de pagina laadt in een echte browser en een opmerking plaatst als de Secure‑SSO gebruiker.