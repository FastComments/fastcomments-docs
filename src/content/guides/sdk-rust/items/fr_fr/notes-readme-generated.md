---
### Identifiants de diffusion

Vous verrez que vous devez passer un `broadcastId` dans certains appels d'API. Lorsque vous recevez des événements, vous récupérerez cet ID, ce qui vous permet d'ignorer l'événement si vous prévoyez d'appliquer les modifications de manière optimiste côté client
(ce que vous souhaiterez probablement faire car cela offre la meilleure expérience). Passez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une même session du navigateur.
---