Pokretljivi showcase nalazi se u [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): aplikacija s lijevom trakom + glavnom pozornicom s po jednom stranicom po widgetu i **stranicom za prijavu koja prikazuje unaprijed postavljene demo korisnike**.  
Prijavite se kao bilo koji od njih i widgeti za komentare i live‑chat autentificiraju taj identitet putem **Secure SSO**. Iz tog direktorija:

```bash
python manage.py migrate
# Koristite vlastiti najmodavac da vidite Secure SSO u akciji (API tajna to omogućuje):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Bez API tajne vraća se na javnog `demo` najmodavca (anonimno).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) je Playwright e2e koji učitava stranicu u stvarnom pregledniku i objavljuje komentar kao Secure-SSO korisnik.