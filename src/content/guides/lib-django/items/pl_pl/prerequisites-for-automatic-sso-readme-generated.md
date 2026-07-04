---
Aby automatycznie przekazać zalogowanego użytkownika do widgetu, tagi odczytują bieżącego użytkownika z żądania. Upewnij się, że Twój projekt ma oba te elementy (są domyślnie włączone w standardowym projekcie Django):

- `django.template.context_processors.request` w `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` w `MIDDLEWARE`

Bez żądania w kontekście szablonu, widgety renderują się dla anonimowego gościa. Zawsze możesz przekazać użytkownika wyraźnie: `{% fastcomments user=some_user %}`.
---