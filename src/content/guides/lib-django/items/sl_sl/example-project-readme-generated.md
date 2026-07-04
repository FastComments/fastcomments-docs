---
Izvedljiva predstavitev se nahaja v [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): lev stolpec + glavni oder  
aplikacija z eno stranjo na pripomoček in **prijavno stranjo, ki prikazuje vnaprej vnešene demo uporabnike**.  
Prijavite se kot kateri koli od njih, in pripomočki za komentarje ter klepet v živo overijo to  
identiteto prek **Secure SSO**. Iz tega direktorija:

```bash
python manage.py migrate
# Uporabite svoj najemnik, da vidite Secure SSO v delovanju (API skrivnost ga omogoča):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Brez API skrivnosti se vrne na javnega `demo` najemnika (anonimen).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) je Playwright e2e  
ki naloži stran v pravem brskalniku in objavi komentar kot Secure-SSO  
uporabnik.
---