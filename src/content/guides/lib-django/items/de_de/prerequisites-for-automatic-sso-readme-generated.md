---
Um den angemeldeten Benutzer automatisch an das Widget zu übergeben, lesen die Tags den  
aktuellen Benutzer aus der Anfrage. Stellen Sie sicher, dass Ihr Projekt beide dieser (sie  
standardmäßig in einem Standard‑Django‑Projekt aktiviert) Komponenten enthält:

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Ohne eine Anfrage im Template-Kontext rendern Widgets für einen anonymen  
Besucher. Sie können immer explizit einen Benutzer übergeben: `{% fastcomments user=some_user %}`.
---