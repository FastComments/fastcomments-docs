Dans l'administration des Webhooks il y a des boutons `Send Test Payload` pour chaque type d'événement (Create, Update, Delete). Les événements Create et Update envoient un objet fictif WebhookComment, tandis que le test de Delete enverra un corps de requête factice ne contenant que un ID.

Le test effectuera deux appels pour vérifier le code de réponse pour les scénarios "happy" (correct API Key) et "sad" (invalid API key).

Lorsque le test envoie une API key invalide, vous devez renvoyer un code d'état 401 pour que le test réussisse complètement. Si vous ne vérifiez pas correctement la valeur du token, vous verrez une erreur.

Ceci permet de s'assurer que vous authentifiez correctement la requête.