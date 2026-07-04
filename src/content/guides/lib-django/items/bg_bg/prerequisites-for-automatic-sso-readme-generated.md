---
За да предадете влезлия потребител на уиджета автоматично, таговете четат текущия потребител от заявката. Уверете се, че вашият проект има и двете (те са включени по подразбиране в стандартен Django проект):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Без заявка в контекста на шаблона, уиджетите се рендерират за анонимен посетител. Винаги можете да предадете потребител изрично: `{% fastcomments user=some_user %}`.
---