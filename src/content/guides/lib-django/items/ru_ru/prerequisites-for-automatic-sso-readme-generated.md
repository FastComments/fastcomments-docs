---
Чтобы автоматически передать вошедшего пользователя в виджет, теги читают  
текущего пользователя из запроса. Убедитесь, что ваш проект содержит оба эти элемента (они  
включены по умолчанию в стандартном проекте Django):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Если в контексте шаблона отсутствует запрос, виджеты отображаются для анонимного  
посетителя. Вы всегда можете передать пользователя явно: `{% fastcomments user=some_user %}`.
---