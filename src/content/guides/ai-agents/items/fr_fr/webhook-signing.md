Chaque webhook d'agent est signé avec HMAC-SHA256 en utilisant le secret API de votre tenant. Le même schéma de signature est utilisé pour les webhooks de commentaires de FastComments - si vous avez déjà intégré ceux-ci, les webhooks d'agent réutilisent le même en-tête de signature et le même flux de vérification.

### Pourquoi la signature

Sans signature, un attaquant qui connaît l'URL de votre webhook pourrait envoyer en POST des événements forgés qui semblent provenir de FastComments. La signature permet à votre endpoint de vérifier que chaque livraison est authentique avant d'agir.

### Fonctionnement des signatures

Pour chaque livraison :

1. La plateforme recherche le secret API pour le tenant + le domaine correspondant (voir [Vue d'ensemble des webhooks](#webhooks-overview)).
2. Elle émet l'horodatage Unix actuel (en millisecondes) dans l'en-tête `X-FastComments-Timestamp`.
3. Elle calcule `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (à la manière de Stripe) et émet le résultat sous la forme `sha256=<hex>` dans l'en-tête `X-FastComments-Signature`.
4. Votre endpoint lit l'en-tête d'horodatage, recalcule le HMAC sur `${timestamp}.${body}` qu'il a reçu, le compare à la valeur `sha256=<hex>` dans l'en-tête de signature et rejette les non-correspondances.

Le corps signé correspond aux **octets exacts** envoyés par la plateforme, préfixés par `${timestamp}.` - votre vérificateur doit utiliser le corps brut de la requête, et non une chaîne JSON re-sérialisée (l'ordre des clés et les espaces seraient sinon différents).

### Secret API

Le même secret API que celui utilisé par les [webhooks de commentaires](/guide-webhooks.html). Il est propre à chaque paire (tenant, domaine) et géré dans les paramètres API de votre tenant. Si vous renouvelez le secret, vous devez redéployer votre vérificateur pour lire la nouvelle valeur avant la prochaine livraison.

Lorsque la plateforme ne trouve **aucun secret API** pour le domaine correspondant, la livraison n'a pas lieu. Le journal des webhooks enregistre l'échec avec la raison « no API secret ».

### Exemple de vérification (Node.js)

[inline-code-attrs-start title = 'Exemple de vérification de signature de webhook'; type='javascript' inline-code-attrs-end]
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

Utilisez `timingSafeEqual` plutôt que `===` pour éviter les fuites de la signature par canal temporel.

### Contenu du corps signé

L'enveloppe complète plus le bloc `data` spécifique à l'événement. Voir [Charges utiles des webhooks](#webhook-payloads).

### Recommandations

- **Vérifiez à chaque livraison.** Si votre endpoint accepte des requêtes non signées, vous n'avez aucune garantie d'intégrité.
- **Rejetez en cas de non-correspondance de signature.** Renvoyez 401 ou 403 ; ne renvoyez pas 200 OK sur une signature invalide, sinon vous masquerez les attaques dans vos journaux de livraison.
- **Utilisez HTTPS.** Les signatures protègent l'intégrité ; TLS protège la confidentialité (à la fois votre secret et le texte des commentaires dans la charge utile).
- **Renouvelez les secrets** lorsque des membres de l'équipe ayant accès partent, ou selon un calendrier régulier.

### Protection contre la relecture

La signature seule ne prévient pas les attaques par relecture - un attaquant qui a capturé une livraison signée authentique peut la renvoyer. La protection contre la relecture relève de votre endpoint :

- Utilisez le champ d'enveloppe `occurredAt` et rejetez les livraisons de plus de, disons, 5 minutes.
- Utilisez `triggerId` ou `approvalId` comme clé de déduplication - si vous l'avez déjà traité, ignorez le doublon.

### Voir aussi

- [Vue d'ensemble des webhooks](#webhooks-overview).
- [Charges utiles des webhooks](#webhook-payloads).
- Le [guide principal des webhooks](/guide-webhooks.html) pour l'infrastructure de signature plus générale.