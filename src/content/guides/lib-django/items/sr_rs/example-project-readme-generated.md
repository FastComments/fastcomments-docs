Покретљив приказ се налази у [`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): апликација са левом траком и главном позорницом
са страницом за сваки виџет и **страном за пријаву која наводи унапред припремљене демо кориснике**.
Пријавите се као било који од њих и виџети за коментаре и живи ћаскању аутентификују тај
идентитет преко **Secure SSO**. Из тог директоријума:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Без API секрета враћа се на јавни `demo` tenant (анонимно).
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) је Playwright e2e
који учитава страницу у правом прегледачу и објављује коментар као Secure-SSO
корисник.