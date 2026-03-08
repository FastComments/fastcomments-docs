#### Accédez à la configuration LTI Canvas

Connectez-vous à votre compte FastComments et allez sur <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Mon compte > Configuration LTI Canvas</a>.

#### Créer une nouvelle configuration LTI

Cochez la case **Activer LTI**. Les champs de configuration apparaîtront :

- **Nom de la configuration** - étiquette optionnelle pour identifier cette configuration (utile si vous connectez plusieurs instances Canvas).
- **URL de la plateforme** - l'URL de votre instance Canvas (p. ex. `https://yourschool.instructure.com`). Vous pouvez laisser ce champ vide pour l'instant et le remplir après avoir créé la clé de développeur.
- **ID client** - laissez ceci vide pour l'instant. Vous le remplirez après avoir créé la clé de développeur dans Canvas.
- **ID de déploiement** - laissez ceci vide pour l'instant.
- **Style de commentaire** - choisissez entre Commentaires, Chat collaboratif ou les deux. Voir [Commenting Styles](#canvas-lms-commenting-styles) pour les détails.

Cliquez sur **Ajouter** pour créer la configuration.

#### Copiez les URL de configuration

Après l'enregistrement, trois URL apparaîtront :

- **URL de configuration** - vous collerez ceci dans Canvas lors de la création de la clé de développeur.
- **OIDC Login URL** - utilisée par Canvas pour le flux de connexion LTI (configurée automatiquement via l'URL de configuration).
- **Launch URL** - le point de terminaison que Canvas appelle lorsqu'un étudiant ouvre FastComments (configuré automatiquement via l'URL de configuration).

Copiez l'**URL de configuration**. Vous en aurez besoin à l'étape suivante.