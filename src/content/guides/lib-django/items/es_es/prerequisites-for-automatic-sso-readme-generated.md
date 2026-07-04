---
Para pasar el usuario conectado al widget automáticamente, las etiquetas leen el  
usuario actual de la solicitud. Asegúrate de que tu proyecto tenga ambos (están  
activados por defecto en un proyecto Django estándar):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Sin una solicitud en el contexto de la plantilla, los widgets se renderizan para un  
visitante anónimo. Siempre puedes pasar un usuario explícitamente: `{% fastcomments user=some_user %}`.
---