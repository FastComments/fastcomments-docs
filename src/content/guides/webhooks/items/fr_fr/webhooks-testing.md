Les nouvelles pages de webhook et les pages d'édition disposent d'un bouton `Send Test Payload` qui envoie une requête à l'URL actuellement dans le formulaire, qu'elle ait été enregistrée ou non. Les événements Create et Update envoient un objet WebhookComment factice, tandis que le test de Delete enverra un corps de requête factice contenant uniquement un ID.

## Vérification des charges utiles

Lors du test de votre intégration webhook, vérifiez que les requêtes entrantes incluent les en‑têtes suivants :

1. **`X-FastComments-Timestamp`** – horodatage Unix (secondes)  
2. **`X-FastComments-Signature`** – signature HMAC‑SHA256  

Les webhooks créés avant l’introduction du schéma de signature reçoivent également un en‑tête **`token`** contenant votre secret d’API. Les nouveaux webhooks ne le font pas.

Utilisez la vérification de signature HMAC pour garantir l’authenticité des charges utiles.

## Outils de test

Vous pouvez utiliser des outils comme [webhook.site](https://webhook.site) ou [ngrok](https://ngrok.com) pour inspecter les charges utiles webhook entrantes pendant le développement.

## Types d'événements

- **Create Event** : déclenché lorsqu’un nouveau commentaire est créé.  
- **Update Event** : déclenché lorsqu’un commentaire est modifié.  
- **Delete Event** : déclenché lorsqu’un commentaire est supprimé.  

Chaque webhook est associé à un seul événement et à une méthode HTTP (POST, PUT ou DELETE). Chaque événement inclut les données complètes du commentaire dans le corps de la requête (voir [Data Structures](/guide-webhooks.html#webhooks-structures) pour le format de la charge utile).

---