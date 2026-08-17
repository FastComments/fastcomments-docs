---
Ein ausführbares Beispiel befindet sich in [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): eine linke Leiste + Hauptbereich‑App mit einer Seite pro Widget und einer **Anmeldeseite, die vorab angelegte Demo‑Benutzer auflistet**.  
Melden Sie sich mit einem beliebigen dieser Benutzer an und die Kommentar‑ und Live‑Chat‑Widgets authentifizieren diese Identität über **Secure SSO**. Aus diesem Verzeichnis:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Ohne ein API‑Geheimnis fällt es auf den öffentlichen `demo`‑Mandanten zurück (anonym).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) ist ein Playwright‑E2E‑Test, der die Seite in einem echten Browser lädt und einen Kommentar als Secure‑SSO‑Benutzer postet.