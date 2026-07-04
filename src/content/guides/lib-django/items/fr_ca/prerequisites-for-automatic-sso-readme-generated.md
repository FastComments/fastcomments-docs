---
Pour transmettre l'utilisateur connecté au widget automatiquement, les balises lisent l'utilisateur actuel à partir de la requête. Assurez‑vous que votre projet possède les deux (ils sont activés par défaut dans un projet Django standard) :

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Sans requête dans le contexte du modèle, les widgets s'affichent pour un visiteur anonyme. Vous pouvez toujours transmettre un utilisateur explicitement : `{% fastcomments user=some_user %}`.
---