Izvršni primer se nalazi u [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): aplikacija sa bočnom trakom i glavnom pozornicom, sa stranicom za svaki vidžet i **stranicom za prijavu koja prikazuje unapred dodate demo korisnike**.
Prijavite se kao bilo koji od njih i vidžeti za komentare i live-chat će autentifikovati taj identitet putem **Secure SSO**. Iz tog direktorijuma:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Bez API tajne vraća se na javni `demo` tenant (anonimno).
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) je Playwright e2e test koji učitava stranicu u stvarnom pregledaču i objavljuje komentar kao Secure-SSO korisnik.