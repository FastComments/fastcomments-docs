---
Per passare l'utente autenticato al widget automaticamente, i tag leggono l'utente corrente dalla request. Assicurati che il tuo progetto abbia entrambi (sono attivi per impostazione predefinita in un progetto Django standard):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Senza una request nel contesto del template, i widget vengono renderizzati per un visitatore anonimo. Puoi sempre passare un utente esplicitamente: `{% fastcomments user=some_user %}`.
---