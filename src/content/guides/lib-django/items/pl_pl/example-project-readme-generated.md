Działający przykład znajduje się w [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): aplikacja z lewym panelem nawigacyjnym i głównym obszarem,
z osobną stroną dla każdego widgetu oraz **stroną logowania zawierającą listę wcześniej utworzonych użytkowników demonstracyjnych**.
Zaloguj się jako dowolny z nich, a widgety komentarzy i czatu na żywo uwierzytelnią tę tożsamość za pomocą **Secure SSO**. Z tego katalogu:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Bez sekretu API aplikacja wraca do publicznego tenanta `demo` (anonimowego).
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) to test e2e Playwright,
który ładuje stronę w prawdziwej przeglądarce i dodaje komentarz jako użytkownik Secure-SSO.