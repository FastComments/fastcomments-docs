### Identifiants de diffusion

Vous verrez qu'il faut transmettre un `broadcastId` dans certaines appels d'API. Lorsque vous recevez des événements, vous récupérerez cet ID, ce qui vous permet d'ignorer l'événement si vous comptez appliquer des modifications de façon optimiste côté client
(ce que vous voudrez probablement faire, car cela offre la meilleure expérience). Transmettez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une session de navigateur.

### SSO (Authentification unique)

Pour des exemples d'authentification unique, voir ci‑dessous.