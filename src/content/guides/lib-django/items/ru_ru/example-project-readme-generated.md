---
Рабочий демонстрационный пример находится в [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): приложение с левой панелью + основной сценой, имеющее страницу для каждого виджета и **страницу входа, перечисляющую заранее подготовленных демо‑пользователей**.  
Войдите под любым из них, и виджеты комментариев и живого чата аутентифицируют эту личность через **Secure SSO**. Из этого каталога:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Без секрета API он переходит к публичному арендатору `demo` (анонимный).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) — это e2e‑тест Playwright, который загружает страницу в реальном браузере и публикует комментарий от имени пользователя Secure-SSO.  
---