### Identifiants de diffusion

Vous verrez que vous devez passer un `broadcastId` dans certains appels d'API. Lorsque vous recevez des événements, vous récupérerez cet ID, ce qui vous permet d'ignorer l'événement si vous envisagez d'appliquer de manière optimiste des modifications côté client
(ce que vous voudrez probablement faire puisque cela offre la meilleure expérience). Fournissez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une session de navigateur.

### SSO (Authentification unique)

Pour des exemples de SSO, voir ci-dessous.