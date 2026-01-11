Dans l'administration Webhooks il y a des boutons `Send Test Payload` pour chaque type d'événement (Create, Update, Delete). Les événements Create et Update envoient un objet WebhookComment factice, tandis que le test de Delete enverra un corps de requête factice contenant uniquement un ID.

Le test effectuera deux appels pour vérifier le code de réponse pour les scénarios "happy" (clé API correcte) et "sad" (clé API invalide).

Lorsque le test envoie une clé API invalide, vous devez renvoyer un code d'état 401 pour que le test réussisse complètement. Si vous ne vérifiez pas correctement la valeur du jeton, vous verrez une erreur.

Ceci permet de s'assurer que vous authentifiez correctement la requête.