---
Pokretljiva demonstracija se nalazi u [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): aplikacija sa levim panelom + glavnom scenom
sa stranicom po widgetu i **stranicom za prijavu koja prikazuje unapred semplovane demo korisnike**.
Prijavite se kao bilo koji od njih i widgeti za komentare i live‑chat autentišu taj identitet putem **Secure SSO**. Iz tog direktorijuma:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Bez API tajne vraća se na javnog `demo` tenant-a (anoniman).
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) je Playwright e2e
koji učitava stranicu u pravom pregledniku i objavljuje komentar kao Secure‑SSO
korisnik.
---