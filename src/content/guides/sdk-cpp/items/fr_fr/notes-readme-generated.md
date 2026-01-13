### Identifiants de diffusion

Vous verrez que vous devez transmettre un `broadcastId` dans certains appels d'API. Lorsque vous recevez des événements, vous récupérerez cet ID, ce qui vous permet d'ignorer l'événement si vous prévoyez d'appliquer les modifications de manière optimiste côté client
(ce que vous voudrez probablement faire car cela offre la meilleure expérience). Transmettez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois dans une session de navigateur.

### SSO (authentification unique)

Pour des exemples de SSO, voir ci-dessous.