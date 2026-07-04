---
Za samodejno posredovanje prijavljenega uporabnika widgetu oznake preberejo trenutnega uporabnika iz zahteve. Prepričajte se, da vaš projekt vsebuje oboje (so privzeto vključeni v standardnem projektu Django):

- `django.template.context_processors.request` v `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` v `MIDDLEWARE`

Brez zahteve v kontekstu predloge se widgeti prikažejo za anonimnega obiskovalca. Vedno lahko uporabnika izrecno posredujete: `{% fastcomments user=some_user %}`.
---