---
Om de ingelogde gebruiker automatisch aan de widget door te geven, lezen de tags de  
huidige gebruiker uit het request. Zorg ervoor dat je project beide heeft (ze  
staan standaard ingeschakeld in een standaard Django-project):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Zonder een request in de template-context renderen widgets voor een anonieme  
bezoeker. Je kunt altijd expliciet een gebruiker doorgeven: `{% fastcomments user=some_user %}`.
---