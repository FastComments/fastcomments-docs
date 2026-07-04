Da biste automatski prenijeli prijavljenog korisnika u widget, tagovi čitaju trenutnog korisnika iz zahtjeva. Provjerite da vaš projekt ima oba navedena elementa (oni su podrazumijevano uključeni u standardnom Django projektu):

- `django.template.context_processors.request` u `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` u `MIDDLEWARE`

Bez zahtjeva u kontekstu šablona, widgeti se renderuju za anonimnog posjetitelja. Uvijek možete eksplicitno proslijediti korisnika: `{% fastcomments user=some_user %}`.