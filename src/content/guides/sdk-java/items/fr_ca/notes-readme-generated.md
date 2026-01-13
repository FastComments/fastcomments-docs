### Identifiants de diffusion

Vous verrez que vous devrez fournir un `broadcastId` dans certaines requêtes d'API. Lorsque vous recevez des événements, vous recevrez cet ID en retour, ainsi vous saurez ignorer l'événement si vous prévoyez d'appliquer des changements de manière optimiste sur le client
(ce que vous voudrez probablement faire puisqu'il offre la meilleure expérience). Fournissez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une session de navigateur.