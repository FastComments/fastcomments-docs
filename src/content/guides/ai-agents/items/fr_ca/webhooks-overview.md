Agent webhooks are HTTP callbacks fired by the platform when an agent run completes or an approval changes state. Use them to forward agent activity into your own systems - moderation dashboards, audit logs, Slack channels, escalation tools.

Configured under the **Webhooks** tab on the [page Agents IA](https://fastcomments.com/auth/my-account/ai-agents).

### Ce qui est livré

Four event types:

- **`trigger.succeeded`** - une exécution d'agent s'est terminée avec succès.
- **`trigger.failed`** - une exécution d'agent a rencontré une erreur.
- **`approval.requested`** - une action a été mise en file d'attente pour approbation humaine.
- **`approval.decided`** - une approbation a été approuvée, rejetée, ou l'exécution a échoué.

Voir [Événements Webhook](#webhook-events) pour savoir quand chaque événement est déclenché, et [Charges utiles des webhooks](#webhook-payloads) pour le schéma de chacun.

### Ce qui n'est pas livré

- **Per-tool-action webhooks.** Une exécution qui appelle `pin_comment` n'envoie pas de webhook séparé pour l'épinglage - l'action est incluse dans la charge utile `trigger.succeeded` de l'exécution. Si vous voulez une livraison par action, analysez le tableau `actions` dans la charge du trigger.
- **Dropped triggers.** Un trigger qui n'est pas dispatché (budget dépassé, portée incorrecte) n'envoie pas de webhook. Les abandons sont visibles uniquement dans la [page Analytique](#analytics-page).
- **Triggers produits par replay.** Les exécutions de test n'envoient pas de webhooks. Voir [Exécutions de test (Replays)](#test-runs-replays).

### Configuration

For each webhook you set:

- **URL** - le point de terminaison HTTPS vers lequel faire un POST.
- **Domain** - le domaine de commentaire auquel ce webhook s'applique (votre tenant peut héberger plusieurs domaines). `*` matches all domains; `*.example.com` is a glob; an exact domain matches only that one.
- **Events** - quels des quatre types d'événements auxquels s'abonner.
- **Agents** - vide pour « tous les agents », ou une liste d'IDs d'agents spécifiques pour filtrer.
- **Method** - POST ou PUT (POST par défaut).
- **No-retry status codes** - codes de réponse HTTP qui doivent être traités comme des échecs définitifs, sans nouvelle tentative (p. ex., 410 Gone, 422 Unprocessable). Voir [Reprises de webhook](#webhook-retries).

Plusieurs webhooks peuvent s'abonner au même événement - chacun est mis en file indépendamment et est livré à son propre URL.

### Correspondance par domaine

A given event is delivered to **every webhook whose `domain` field matches the comment's domain**. The matching uses a simple glob:

- Exact: `example.com` matches only `example.com`.
- Wildcard star: `*` matches every domain.
- Subdomain glob: `*.example.com` matches `blog.example.com`, `forum.example.com`, but not `example.com` itself.

Le domaine présent dans une charge utile est le premier résultat non nul parmi : le commentaire `domain`, l'URL analysée par rapport à la configuration de domaine de votre tenant, ou la recherche `Page` par `urlId`.

### Filtrage par agent

The **Agents** field lets a webhook subscribe to only certain agents. Empty means "all agents". When non-empty, the webhook only fires for agents in the list.

Ceci est utile lorsque vous avez un webhook pour les événements de modération et un autre pour les événements d'engagement, chacun acheminant vers des systèmes en aval différents.

### Envoi de test

The webhook config UI has a **Test send** button that posts a fake payload to the URL so you can verify connectivity, signing, and your endpoint's response code without waiting for a real event.

### Journaux de livraison

Every delivery (and every retry) lands in the [Webhook Delivery Logs](#webhook-logs) page so you can see what happened on the wire.

### Signature

Every webhook is signed with HMAC-SHA256 using your tenant's API secret. See [Webhook Signing](#webhook-signing).

### Éligibilité

Agent webhooks require valid billing on the tenant. They use the same signing/secret infrastructure as your existing comment webhooks - if you have already integrated comment webhooks (see the [guide Webhooks](/guide-webhooks.html)), the agent webhook integration is the same shape with new event types.