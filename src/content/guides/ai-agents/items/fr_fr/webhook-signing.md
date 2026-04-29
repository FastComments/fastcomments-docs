Chaque webhook d'agent est signé avec HMAC-SHA256 en utilisant le secret API de votre locataire. Le même schéma de signature est utilisé pour les webhooks de commentaires de FastComments : si vous les avez déjà intégrés, les webhooks d'agent réutilisent le même en-tête de signature et le même flux de vérification.

### Why signing

Sans signature, un attaquant qui connaît votre URL de webhook pourrait POSTer des événements falsifiés ressemblant à ceux envoyés par FastComments. La signature permet à votre endpoint de vérifier que chaque livraison est authentique avant d'agir.

### How signatures work

Pour chaque livraison :

1. La plateforme recherche le secret API pour le locataire + le domaine correspondant (voir [Présentation des webhooks](#webhooks-overview)).
2. Elle émet l'horodatage Unix courant (en millisecondes) dans l'en-tête `X-FastComments-Timestamp`.
3. Elle calcule `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (à la manière de Stripe) et émet le résultat sous la forme `sha256=<hex>` dans l'en-tête `X-FastComments-Signature`.
4. Votre endpoint lit l'en-tête d'horodatage, recalculer l'HMAC sur `${timestamp}.${body}` qu'il a reçu, compare avec la valeur `sha256=<hex>` dans l'en-tête de signature, et rejette les divergences.

Le corps qui est signé correspond aux **octets exacts** envoyés par la plateforme, préfixés par `${timestamp}.` - votre vérificateur doit utiliser le corps brut de la requête, et non une chaîne JSON re-sérialisée (l'ordre des clés et les espaces blancs seraient alors différents).

### API secret

Le même secret API utilisé par les [comment webhooks](/guide-webhooks.html). Il est par (tenant, domaine) et géré dans les paramètres API de votre locataire. Si vous faites pivoter le secret, vous devez redéployer votre vérificateur pour qu'il lise la nouvelle valeur avant la prochaine livraison.

Lorsque la plateforme ne trouve **aucun secret API** pour le domaine correspondant, la livraison n'a pas lieu. Le journal des webhooks enregistre l'échec avec la raison "no API secret".

### Verification example (Node.js)

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

### What's in the signed body

L'enveloppe complète plus le bloc `data` spécifique à l'événement. Voir [Charges utiles des webhooks](#webhook-payloads).

### Recommendations

- **Vérifiez à chaque livraison.** Si votre endpoint accepte des requêtes non signées, vous n'avez aucune garantie d'intégrité.
- **Rejetez en cas de divergence de signature.** Retournez 401 ou 403 ; ne renvoyez pas 200 OK sur une mauvaise signature, sinon vous masquerez des attaques dans vos journaux de livraison.
- **Utilisez HTTPS.** Les signatures protègent l'intégrité ; TLS protège la confidentialité (à la fois de votre secret et du texte du commentaire dans la payload).
- **Faites pivoter les secrets** lorsque des membres de l'équipe ayant accès partent, ou selon un calendrier.

### Replay protection

La signature seule ne prévient pas les attaques par rejeu : un attaquant ayant capturé une livraison signée réelle peut la renvoyer. La protection contre le rejeu dépend de votre endpoint :

- Utilisez le champ d'enveloppe `occurredAt` et rejetez les livraisons plus anciennes que, par exemple, 5 minutes.
- Utilisez `triggerId` ou `approvalId` comme clé de déduplication : si vous l'avez déjà traité, ignorez le doublon.

### See also

- [Présentation des webhooks](#webhooks-overview).
- [Charges utiles des webhooks](#webhook-payloads).
- Le [guide principal des webhooks](/guide-webhooks.html) pour l'infrastructure de signature plus large.