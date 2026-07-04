---
Pour transmettre l'utilisateur connecté au widget automatiquement, les balises lisent le
utilisateur actuel depuis la requête. Assurez‑vous que votre projet possède les deux (elles
sont activées par défaut dans un projet Django standard) :

- `django.template.context_processors.request` dans `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` dans `MIDDLEWARE`

Sans une requête dans le contexte du modèle, les widgets s'affichent pour un visiteur anonyme.
Vous pouvez toujours passer un utilisateur explicitement : `{% fastcomments user=some_user %}`.
---