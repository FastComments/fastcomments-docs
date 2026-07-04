---
Una demo eseguibile si trova in [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): un'app a colonna laterale sinistra + stage principale con una pagina per widget e una **pagina di accesso che elenca gli utenti demo pre‑caricati**.  
Accedi con uno qualsiasi di loro e i widget di commento e chat live autenticano quell’identità tramite **Secure SSO**. Da quella directory:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Senza una chiave API segreta, ricade sul tenant pubblico `demo` (anonimo).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) è un test end‑to‑end Playwright che carica la pagina in un browser reale e pubblica un commento come utente Secure‑SSO.