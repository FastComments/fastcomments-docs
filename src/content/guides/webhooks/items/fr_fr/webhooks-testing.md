---
Dans l'administration Webhooks il y a des boutons `Send Test Payload` pour chaque type d'événement (Create, Update, Delete). Les événements Create et Update envoient un objet WebhookComment factice, tandis que le test Delete enverra un corps de requête factice contenant juste un ID.

## Vérification des charges utiles

Lorsque vous testez votre intégration de webhook, vérifiez que les requêtes entrantes incluent les en-têtes suivants :

1. **`token`** - Votre secret API
2. **`X-FastComments-Timestamp`** - horodatage Unix (secondes)
3. **`X-FastComments-Signature`** - signature HMAC-SHA256

Utilisez la vérification de la signature HMAC pour garantir que les charges utiles sont authentiques.

## Outils de test

Vous pouvez utiliser des outils comme [webhook.site](https://webhook.site) ou [ngrok](https://ngrok.com) pour inspecter les charges utiles entrantes des webhooks pendant le développement.

## Types d'événements

- **Create Event**: Déclenché lorsqu'un nouveau commentaire est créé. Méthode par défaut : PUT
- **Update Event**: Déclenché lorsqu'un commentaire est édité. Méthode par défaut : PUT
- **Delete Event**: Déclenché lorsqu'un commentaire est supprimé. Méthode par défaut : DELETE

Chaque événement inclut l'ensemble des données du commentaire dans le corps de la requête (voir [Structures de données](/guides/webhooks/webhooks-structures) pour le format de la charge utile).

---