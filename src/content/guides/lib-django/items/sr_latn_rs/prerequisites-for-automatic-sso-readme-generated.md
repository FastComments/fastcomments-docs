Da biste automatski prosledili ulogovanog korisnika widgetu, oznake čitaju  
trenutnog korisnika iz zahteva. Uverite se da vaš projekat ima oba (ona su  
podrazumevano uključena u standardnom Django projektu):

- `django.template.context_processors.request` u `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` u `MIDDLEWARE`

Bez zahteva u kontekstu šablona, widgeti se renderuju za anonimnog  
posetioca. Uvek možete eksplicitno proslediti korisnika: `{% fastcomments user=some_user %}`.