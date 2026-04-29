Chaque webhook d'agent est signé avec HMAC-SHA256 en utilisant le secret API de votre locataire. Le même schéma de signature est utilisé pour les webhooks de commentaires de FastComments — si vous avez déjà intégré ceux-ci, les webhooks d'agent réutilisent le même en-tête de signature et le même flux de vérification.

### Pourquoi la signature

Sans signature, un attaquant qui connaît votre URL de webhook pourrait POSTer des événements forgés semblant provenir de FastComments. La signature permet à votre point de terminaison de vérifier que chaque livraison est authentique avant d'agir.

### Comment fonctionnent les signatures

Pour chaque livraison :

1. La plateforme recherche le secret API pour le locataire + domaine correspondant (voir [Webhooks Overview](#webhooks-overview)).
2. Elle émet l'horodatage Unix courant (en millisecondes) dans l'en-tête `X-FastComments-Timestamp`.
3. Elle calcule `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (à la Stripe) et émet le résultat sous la forme `sha256=<hex>` dans l'en-tête `X-FastComments-Signature`.
4. Votre point de terminaison lit l'en-tête d'horodatage, recompute le HMAC sur `${timestamp}.${body}` qu'il a reçu, compare avec la valeur `sha256=<hex>` dans l'en-tête de signature, et rejette les non-correspondances.

Le corps qui est signé est les **octets exacts** envoyés par la plateforme, préfixés par `${timestamp}.` — votre vérificateur doit utiliser le corps de requête brut, pas une chaîne JSON re-sérialisée (l'ordre des clés et les espaces blancs seraient alors différents).

### Secret API

Le même Secret API utilisé par [comment webhooks](/guide-webhooks.html). Il est par (locataire, domaine) et géré dans les paramètres API de votre locataire. Si vous faites une rotation du secret, vous devez redéployer votre vérificateur pour lire la nouvelle valeur avant la prochaine livraison.

Lorsque la plateforme ne trouve **no API secret** pour le domaine correspondant, la livraison n'a pas lieu. Le journal des webhooks enregistre l'échec avec la raison "no API secret".

### Exemple de vérification (Node.js)

[inline-code-attrs-start title = 'Exemple de vérification de la signature du webhook'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Utilisez `timingSafeEqual` plutôt que `===` pour éviter les fuites par canaux temporels de la signature.

### Que contient le corps signé

L'enveloppe complète plus le bloc `data` spécifique à l'événement. Voir [Webhook Payloads](#webhook-payloads).

### Recommandations

- **Vérifiez à chaque livraison.** Si votre point de terminaison accepte des requêtes non signées, vous n'avez aucune garantie d'intégrité.
- **Rejetez en cas de non-correspondance de signature.** Retournez 401 ou 403 ; ne renvoyez pas 200 OK pour une mauvaise signature, sinon vous masquerez des attaques dans vos journaux de livraison.
- **Utilisez HTTPS.** Les signatures protègent l'intégrité ; TLS protège la confidentialité (à la fois votre secret et le texte du commentaire dans la charge utile).
- **Faites une rotation des secrets** lorsque des membres de l'équipe ayant accès partent, ou selon un calendrier.

### Protection contre les attaques par rejeu

La signature à elle seule n'empêche pas les attaques par rejeu — un attaquant qui a capturé une livraison signée réelle peut la renvoyer. La protection contre le rejeu dépend de votre point de terminaison :

- Utilisez le champ d'enveloppe `occurredAt` et rejetez les livraisons plus anciennes que, par exemple, 5 minutes.
- Utilisez `triggerId` ou `approvalId` comme clé de dédoublonnage — si vous l'avez déjà traité, ignorez le doublon.

### Voir aussi

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Le guide principal [Webhooks guide](/guide-webhooks.html) pour l'infrastructure de signature plus générale.