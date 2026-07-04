For at automatisk videregive den indloggede bruger til widgeten, læser tag'ene den aktuelle bruger fra request'en. Sørg for, at dit projekt har begge disse (de er som standard aktiveret i et standard Django‑projekt):

- `django.template.context_processors.request` i `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` i `MIDDLEWARE`

Uden en request i skabelonens kontekst, renderes widgets for en anonym besøgende. Du kan altid videregive en bruger eksplicit: `{% fastcomments user=some_user %}`.