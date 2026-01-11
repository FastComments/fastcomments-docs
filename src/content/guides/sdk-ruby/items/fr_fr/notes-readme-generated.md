### Identifiants de diffusion

Vous verrez que vous devez fournir un `broadcastId` dans certains appels d'API. Lorsque vous recevrez des événements, cet ID vous sera renvoyé, ce qui vous permet d'ignorer l'événement si vous prévoyez d'appliquer les changements de manière optimiste côté client
(ce que vous voudrez probablement faire car cela offre la meilleure expérience). Fournissez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois lors d'une session de navigateur.

### SSO (authentification unique)

Pour des exemples de SSO, voir ci-dessous.