### Identifiants de diffusion

Vous verrez que vous devez passer un `broadcastId` dans certaines requêtes API. Lorsque vous recevrez des événements, vous récupérerez cet ID, ce qui vous permettra d'ignorer l'événement si vous comptez appliquer les changements de façon optimiste côté client (ce que vous voudrez probablement faire car cela offre la meilleure expérience). Passez un `UUID` ici. L'ID doit être suffisamment unique pour ne pas se produire deux fois dans une session de navigateur.

### SSO (Authentification unique)

Pour des exemples de SSO, voir ci-dessous.