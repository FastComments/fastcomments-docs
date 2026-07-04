---
Kako bi se automatski proslijedio prijavljeni korisnik widgetu, oznake čitaju trenutnog korisnika iz zahtjeva. Provjerite da vaš projekt ima oboje (standardni Django projekt ih ima po zadanom):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Bez zahtjeva u kontekstu predloška, widgeti se renderaju za anonimnog posjetitelja. Uvijek možete izričito proslijediti korisnika: `{% fastcomments user=some_user %}`.
---