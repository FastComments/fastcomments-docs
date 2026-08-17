---
Да бисте аутоматски прослали пријављеног корисника у виџет, ознаке читају тренутног корисника из захтева. Уверите се да ваш пројекат има оба ова (они су подразумевано укључени у стандардном Django пројекту):

- `django.template.context_processors.request` у `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` у `MIDDLEWARE`

Без захтева у контексту шаблона, виџети се приказују за анонимног посетилаца. Увек можете експлицитно проследити корисника: `{% fastcomments user=some_user %}`.
---