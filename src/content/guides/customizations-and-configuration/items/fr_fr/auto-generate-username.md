Lorsque les utilisateurs commentent ou votent et qu'ils ne sont pas connectés, on leur demandera de fournir leur e‑mail et leur nom d'utilisateur.

Pour certains sites, demander à un visiteur d'inventer un nom d'utilisateur unique constitue un obstacle, notamment sur mobile. FastComments peut
générer un nom d'utilisateur neutre pour chaque nouveau visiteur et le préremplir dans le champ du nom d'utilisateur, comme `BraveOtter4172`.

Le visiteur peut le laisser tel quel, ou le remplacer par un nom de son choix.

Cela peut être activé depuis l'interface de personnalisation, sous le paramètre appelé `Generate Usernames Automatically` :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='L\'option Générer les noms d\'utilisateur automatiquement dans l\'interface de personnalisation du widget'; title='Générer les noms d\'utilisateur automatiquement' app-screenshot-end]

#### Fonctionnement

- Chaque nom généré est unique. Il est vérifié par rapport aux comptes existants et réservé pour la session du navigateur de ce visiteur, de sorte que deux visiteurs ne se voient pas proposer le même nom.
- Le nom n'est généré que pour les visiteurs qui n'en ont pas encore. Les utilisateurs connectés, les utilisateurs SSO et les visiteurs qui ont déjà commenté conservent leur nom existant.
- Cela fonctionne avec ou sans [commentaire anonyme](/guide-customizations-and-configuration.html#allow-anon). Lorsque le commentaire anonyme est désactivé, le visiteur saisit toujours son e‑mail, mais n'a plus besoin de réfléchir à un nom d'utilisateur.
- Un visiteur de retour qui saisit un e‑mail qu'il a déjà utilisé est associé à son compte existant et conserve le nom de ce compte.
- Si un [Nom d'utilisateur par défaut](/guide-customizations-and-configuration.html#default-username) est également défini, le nom généré prend le pas.