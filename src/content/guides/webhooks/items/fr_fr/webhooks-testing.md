Dans l'interface d'administration des Webhooks, il existe des boutons `Send Test Payload` pour chaque type d'événement (Create, Update, Delete). Les événements Create et Update envoient un objet WebhookComment factice, tandis que le test de Delete enverra un corps de requête factice contenant uniquement un ID.

## Vérification des payloads

Lors des tests de votre intégration de webhook, vérifiez que les requêtes entrantes incluent les en-têtes suivants :

1. **`token`** - Votre API Secret
2. **`X-FastComments-Timestamp`** - Horodatage Unix (secondes)
3. **`X-FastComments-Signature`** - Signature HMAC-SHA256

Utilisez la vérification de la signature HMAC pour garantir l'authenticité des payloads.

## Outils de test

Vous pouvez utiliser des outils comme [webhook.site](https://webhook.site) ou [ngrok](https://ngrok.com) pour inspecter les payloads de webhook entrants pendant le développement.

## Types d'événements

- **Create Event**: Déclenché lorsqu'un nouveau commentaire est créé. Méthode par défaut : PUT
- **Update Event**: Déclenché lorsqu'un commentaire est modifié. Méthode par défaut : PUT
- **Delete Event**: Déclenché lorsqu'un commentaire est supprimé. Méthode par défaut : DELETE

Chaque événement inclut l'ensemble des données du commentaire dans le corps de la requête (voir [Structures de données](/guide-webhooks.html#webhooks-structures) pour le format du payload).