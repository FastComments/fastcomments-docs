---
Щоб автоматично передати залогіненого користувача у віджет, теги читають поточного
користувача з запиту. Переконайтеся, що ваш проект має обидва ці налаштування (вони
увімкнені за замовчуванням у стандартному проекті Django):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Якщо в контексті шаблону немає запиту, віджети відображаються для анонімного
відвідувача. Ви завжди можете передати користувача явно: `{% fastcomments user=some_user %}`.
---