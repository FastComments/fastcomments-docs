Запусковий приклад знаходиться в [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): застосунок з лівою панеллю навігації та основною сценою, зі сторінкою для кожного віджета та **сторінкою входу зі списком попередньо створених демо-користувачів**.
Увійдіть як будь-хто з них, і віджети коментарів та живого чату автентифікують цю особу через **Secure SSO**. З цього каталогу:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Без API-секрету відбувається повернення до публічного орендаря `demo` (анонімний режим).
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) — це Playwright e2e-тест, який завантажує сторінку в реальному браузері та публікує коментар від імені користувача Secure SSO.