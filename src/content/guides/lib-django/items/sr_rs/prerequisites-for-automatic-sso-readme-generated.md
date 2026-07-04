---
Da biste automatski prosledili prijavljenog korisnika widgetu, tagovi čitaju
trenutnog korisnika iz zahteva. Uverite se da vaš projekat ima oba ova
podešavanja (podrazumevano su uključena u standardnom Django projektu):

- `django.template.context_processors.request` u `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` u `MIDDLEWARE`

Bez zahteva u kontekstu šablona, widgeti se renderuju za anonimnog
posetioca. Uvek možete izričito proslediti korisnika: `{% fastcomments user=some_user %}`.
---