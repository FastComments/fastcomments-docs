Работещ демонстрационен пример се намира в [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): приложение с ляво навигационно меню + главна сцена, с по една страница за всеки уиджет и **страница за вход, изброяваща предварително зададени демо потребители**. Влезте като който и да е от тях и уиджетите за коментари и жив чат удостоверяват тази идентичност чрез **Secure SSO**. От тази директория:

```bash
python manage.py migrate
# Използвайте ваш собствен наемател, за да видите Secure SSO в действие (API тайна го активира):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Без API тайна се връща към публичния `demo` наемател (анонимен). [`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) е Playwright e2e, който зарежда страницата в реален браузър и публикува коментар като потребител на Secure-SSO.