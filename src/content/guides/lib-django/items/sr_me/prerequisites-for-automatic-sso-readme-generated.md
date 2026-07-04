---
Da biste automatski proslijedili ulogovanog korisnika widgetu, oznake čitaju
trenutnog korisnika iz zahtjeva. Uverite se da vaš projekt ima oba ova (ona
su podrazumevano uključena u standardnom Django projektu):

- `django.template.context_processors.request` u `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` u `MIDDLEWARE`

Bez zahtjeva u kontekstu šablona, widgeti se renderuju za anonimnog
posjetioca. Uvek možete izričito proslijediti korisnika: `{% fastcomments user=some_user %}`.
---