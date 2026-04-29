Chaque webhook d'agent possède son propre journal de livraison. Accessible depuis la [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) via le bouton **Logs** sur chaque ligne de webhook.

### Ce qui se trouve sur la page

Un tableau paginé avec une ligne par tentative de livraison :

| Colonne | Signification |
|---|---|
| Quand | Quand la tentative a eu lieu. |
| Événement | Le type d'événement (`trigger.succeeded`, `approval.requested`, etc.). |
| Statut | Le statut de la livraison. |
| StatusCode | Le code d'état HTTP retourné par votre endpoint, lorsqu'il est disponible. |
| Description | Une courte description du résultat. Pour les livraisons échouées où aucune réponse HTTP n'a été reçue, le message d'erreur sous-jacent est enregistré sous la forme `{error: <message>}`. |

La page ne prend en charge que la pagination — il n'y a pas de filtres par statut, type d'événement ou plage de dates.

### Ce que vous pouvez faire à partir des journaux

- **Décider si un code de statut doit être dans No-retry.** Si vous voyez votre endpoint renvoyer le même `4xx` encore et encore, c'est un signal fort que vous voulez l'ajouter aux **No-retry status codes** afin que la plateforme cesse de réessayer.

### Informations sur les échecs

Lorsque une livraison échoue sans réponse HTTP (échec DNS, connexion refusée, délai d'attente, erreur TLS, etc.), le message d'erreur brut est enregistré sous la forme `{error: <message>}`. La plateforme ne classe pas ces erreurs en catégories nommées comme `TIMEOUT` ou `DNS_ERROR` — lisez directement le message d'erreur pour voir ce qui s'est passé.

Pour les réponses HTTP, la colonne StatusCode affiche le code renvoyé par votre endpoint. Cas courants :

- **Toutes les tentatives : `401` ou `403`** - votre endpoint rejette la signature. Vérifiez que vous calculez le HMAC sur `${timestamp}.${body}` et que vous utilisez le bon secret. Voir [Signature des webhooks](#webhook-signing).
- **Toutes les tentatives : `422`** - votre endpoint pense que la charge utile est invalide. Soit corrigez votre endpoint, soit ajoutez `422` aux **No-retry status codes** si le rejet est attendu pour certains événements.

### Contexte par livraison

Chaque entrée de journal contient :

- `webhookId` - quelle configuration de webhook a produit cette livraison.
- `agentId` - à quel agent la livraison se rapporte.
- `triggerId` or `approvalId` - l'enregistrement sous-jacent.
- `domain` - le domaine correspondant.

Vous pouvez les utiliser pour corréler une livraison échouée avec l'exécution à laquelle elle se rapporte dans [Historique des exécutions](#run-history).

### Rétention

Les entrées `AgentSyncLog` ont un TTL fixe d'un an sur `createdAt` indépendamment du résultat — les livraisons réussies et échouées sont conservées pendant la même durée. Une fois la période de rétention dépassée, l'entrée de journal est supprimée.

Si vous avez besoin d'un audit à long terme, la pratique durable est la suivante : faites en sorte que le **endpoint lui-même** persiste les livraisons qu'il reçoit. Cela découple votre journal d'audit de la politique de rétention de la plateforme.

### Envoi de test

Le bouton **Test send** du formulaire de configuration du webhook écrit une fausse livraison dans la même table de journal afin que vous puissiez vérifier la connectivité de bout en bout avant de vous fier à des événements réels. Les livraisons de test sont clairement étiquetées dans le journal afin qu'elles ne polluent pas les statistiques d'échec de production.

### Voir aussi

- [Aperçu des webhooks](#webhooks-overview).
- [Webhook Retries](#webhook-retries) pour la sémantique de réessai qui alimente ces journaux.
- [Signature des webhooks](#webhook-signing) pour savoir comment vérifier sur votre endpoint.