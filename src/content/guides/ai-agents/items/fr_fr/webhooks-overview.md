Les webhooks d'agent sont des callbacks HTTP déclenchés par la plateforme lorsqu'une exécution d'agent se termine ou lorsqu'une approbation change d'état. Utilisez-les pour transmettre l'activité des agents à vos propres systèmes - tableaux de bord de modération, journaux d'audit, chaînes Slack, outils d'escalade.

Configurés sous l'onglet **Webhooks** sur la [page Agents IA](https://fastcomments.com/auth/my-account/ai-agents).

### Ce qui est livré

Quatre types d'événements :

- **`trigger.succeeded`** - une exécution d'agent s'est terminée avec succès.
- **`trigger.failed`** - une exécution d'agent a rencontré une erreur.
- **`approval.requested`** - une action a été mise en file d'attente pour approbation humaine.
- **`approval.decided`** - une approbation a été approuvée, rejetée ou l'exécution a échoué.

Voir [Événements Webhook](#webhook-events) pour savoir quels événements se déclenchent et quand, et [Charges utiles des webhooks](#webhook-payloads) pour le schéma de chacun.

### Ce qui n'est pas livré

- **Webhooks par action d'outil.** Une exécution qui appelle `pin_comment` n'enverra pas de webhook distinct pour l'épinglage - l'action est incluse dans la charge utile `trigger.succeeded` de l'exécution. Si vous voulez une livraison par action, analysez le tableau `actions` dans la charge utile du trigger.
- **Triggers abandonnés.** Un trigger qui n'est pas dispatché (budget dépassé, périmètre incorrect) n'enverra pas de webhook. Les abandons sont visibles uniquement dans la [page Analytics](#analytics-page).
- **Triggers produits par replay.** Les exécutions de test n'envoient pas de webhooks. Voir [Exécutions de test (Replays)](#test-runs-replays).

### Configuration

Pour chaque webhook que vous configurez :

- **URL** - le point de terminaison HTTPS vers lequel effectuer le POST.
- **Domain** - le domaine de commentaire auquel ce webhook s'applique (votre tenant peut héberger plusieurs domaines). `*` correspond à tous les domaines ; `*.example.com` est un glob ; un domaine exact ne correspond qu'à lui-même.
- **Events** - lesquels des quatre types d'événement auxquels s'abonner.
- **Agents** - vide pour « tous les agents », ou une liste d'identifiants d'agents spécifiques à filtrer.
- **Method** - POST ou PUT (POST par défaut).
- **No-retry status codes** - codes de réponse HTTP à considérer comme des échecs définitifs, non réessayés (par ex., 410 Gone, 422 Unprocessable). Voir [Réessais des webhooks](#webhook-retries).

Plusieurs webhooks peuvent s'abonner au même événement - chacun est mis en file indépendamment et est livré à sa propre URL.

### Correspondance par domaine

Un événement donné est livré à **chaque webhook dont le champ `domain` correspond au domaine du commentaire**. La correspondance utilise un glob simple :

- Exact : `example.com` ne correspond qu'à `example.com`.
- Astérisque (joker) : `*` correspond à tous les domaines.
- Glob de sous-domaine : `*.example.com` correspond à `blog.example.com`, `forum.example.com`, mais pas à `example.com` lui-même.

Le domaine présent dans une payload est le premier résultat non nul parmi : le `domain` du commentaire, l'URL analysée par rapport à la configuration de domaine de votre tenant, ou la recherche `Page` par `urlId`.

### Filtrage par agent

Le champ **Agents** permet à un webhook de ne s'abonner qu'à certains agents. Vide signifie « tous les agents ». Lorsqu'il n'est pas vide, le webhook ne se déclenche que pour les agents figurant dans la liste.

Ceci est utile lorsque vous avez un webhook pour les événements de modération et un autre pour les événements d'engagement, chacun étant routé vers des systèmes en aval différents.

### Envoi de test

L'interface de configuration du webhook dispose d'un bouton **Envoi de test** qui envoie une payload factice vers l'URL afin que vous puissiez vérifier la connectivité, la signature et le code de réponse de votre point de terminaison sans attendre un événement réel.

### Journaux de livraison

Chaque livraison (et chaque nouvelle tentative) apparaît dans la page [Journaux de livraison des webhooks](#webhook-logs) afin que vous puissiez voir ce qui s'est passé sur le réseau.

### Signature

Chaque webhook est signé avec HMAC-SHA256 en utilisant le secret API de votre tenant. Voir [Signature des webhooks](#webhook-signing).

### Éligibilité

Les webhooks d'agent requièrent une facturation valide sur le tenant. Ils utilisent la même infrastructure de signature/secret que vos webhooks de commentaire existants - si vous avez déjà intégré les webhooks de commentaire (voir le [guide Webhooks](/guide-webhooks.html)), l'intégration des webhooks d'agent a la même forme avec de nouveaux types d'événements.