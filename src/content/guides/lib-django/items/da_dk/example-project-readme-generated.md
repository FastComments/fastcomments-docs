Et kørbart showcase findes i [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): en app med venstre-menu og hovedvindue,
med en side pr. widget og en **log-ind-side med forudoprettede demo-brugere**.
Log ind som en af dem, og kommentar- og live-chat-widgetterne autentificerer
denne identitet via **Secure SSO**. Fra den mappe:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Uden en API-hemmelighed falder den tilbage til den offentlige `demo`-tenant (anonym).
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) er en Playwright e2e,
der indlæser siden i en rigtig browser og poster en kommentar som Secure-SSO-brugeren.