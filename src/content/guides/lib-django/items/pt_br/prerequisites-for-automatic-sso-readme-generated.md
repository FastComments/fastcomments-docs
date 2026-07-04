---
Para passar o usuário conectado ao widget automaticamente, as tags leem o
usuário atual a partir da requisição. Certifique‑se de que seu projeto possui ambos (eles
estão ativados por padrão em um projeto Django padrão):

- `django.template.context_processors.request` em `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` em `MIDDLEWARE`

Sem uma requisição no contexto do template, os widgets são renderizados para um visitante
anônimo. Você sempre pode passar um usuário explicitamente: `{% fastcomments user=some_user %}`.
---