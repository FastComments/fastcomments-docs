### IDs de diffusion

Vous verrez qu'il faut passer un `broadcastId` dans certains appels d'API. Quand vous recevrez des événements, vous obtiendrez ce même ID en retour, ce qui vous permet d'ignorer l'événement si vous prévoyez d'appliquer les changements de façon optimiste côté client
(ce que vous voudrez probablement faire, car cela offre la meilleure expérience). Passez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois lors d'une session de navigateur.