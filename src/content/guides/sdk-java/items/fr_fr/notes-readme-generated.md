### Identifiants de diffusion

Vous verrez qu'il faut passer un `broadcastId` dans certains appels d'API. Lorsque vous recevez des événements, vous récupérerez cet ID, ce qui vous permettra d'ignorer l'événement si vous prévoyez d'appliquer de manière optimiste des modifications côté client
(ce que vous voudrez probablement faire car cela offre la meilleure expérience). Passez un UUID ici. L'ID doit être suffisamment unique pour ne pas se produire deux fois dans une session de navigateur.