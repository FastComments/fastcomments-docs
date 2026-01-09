### Identifiants de diffusion

Vous verrez que vous devez transmettre un `broadcastId` dans certains appels d'API. Lorsque vous recevez des événements, vous recevrez cet ID en retour, ce qui vous permet de savoir quand ignorer l'événement si vous prévoyez d'appliquer des modifications de façon optimiste côté client
(ce que vous voudrez probablement faire, car cela offre la meilleure expérience). Transmettez un `UUID` ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une session de navigateur.

### SSO (Authentification unique)

Pour des exemples d'authentification unique, voir ci‑dessous.