#### Accéder à Canvas LTI Config

Connectez-vous à votre compte FastComments et allez à <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Cochez la case **Enable LTI**. Les champs de configuration apparaîtront :

- **Configuration Name** - une étiquette facultative pour identifier cette configuration (utile si vous connectez plusieurs instances Canvas).
- **Platform URL** - l'URL de votre instance Canvas (par ex. `https://yourschool.instructure.com`). Vous pouvez laisser ceci vide pour l'instant et le remplir après avoir créé la Developer Key.
- **Client ID** - laissez ceci vide pour l'instant. Vous le remplirez après avoir créé la Developer Key dans Canvas.
- **Deployment ID** - laissez ceci vide pour l'instant.
- **Comment Style** - choisissez entre Comments, Collab Chat, ou Both. Voir [Commenting Styles](#canvas-lms-commenting-styles) pour les détails.

Cliquez sur **Add** pour créer la configuration.

#### Copy the Configuration URLs

Après l'enregistrement, trois URL apparaîtront :

- **Configuration URL** - vous collerez ceci dans Canvas lors de la création de la Developer Key.
- **OIDC Login URL** - utilisé par Canvas pour le flux de connexion LTI (configuré automatiquement via la Configuration URL).
- **Launch URL** - le point de terminaison appelé par Canvas lorsqu'un étudiant ouvre FastComments (configuré automatiquement via la Configuration URL).

Copiez la **Configuration URL**. Vous en aurez besoin à l'étape suivante.