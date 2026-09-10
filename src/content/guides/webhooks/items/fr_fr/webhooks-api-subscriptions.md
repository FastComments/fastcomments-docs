Webhooks peuvent également être gérés via l’API REST. C’est ainsi que des intégrations comme Zapier s’abonnent aux événements de commentaire sans toucher au tableau de bord, et cela suit le modèle REST Hooks : s’abonner, recevoir des événements, se désabonner.

Les abonnements API coexistent avec les webhooks configurés dans le tableau de bord. Un événement de commentaire est livré au webhook du tableau de bord pour son domaine et à chaque abonnement API qui correspond, chacun comme sa propre livraison. Il n’y a aucune limite d’un abonné par événement.

## Authentification

Chaque requête doit contenir votre clé API dans l’en‑tête `x-api-key` (ou le paramètre de requête `API_KEY`) et votre ID de locataire dans le paramètre de requête `tenantId`. Les deux sont affichés sur la page Secret API du tableau de bord.

## Souscrire

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Champ | Obligatoire | Description |
|-------|-------------|-------------|
| `url` | Oui | Une URL absolue http ou https. |
| `event` | Oui | `comment-created`, `comment-updated` ou `comment-deleted`. |
| `domain` | Non | Un domaine de la configuration de votre compte. La valeur par défaut est `*`, qui reçoit les événements pour chaque domaine. |
| `method` | Non | `POST` (par défaut), `PUT` ou `DELETE`. |

La réponse contient l’abonnement :

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Souscrire la même URL au même événement et domaine renvoie l’abonnement existant plutôt que de créer un doublon, ce qui permet à un client de réessayer en toute sécurité. Chaque locataire peut avoir jusqu’à 50 abonnements API.

## Liste

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Renvoie chaque webhook du locataire, y compris ceux gérés dans le tableau de bord (`"source": "dashboard"`). Filtrez avec `event`, `domain` ou `source`.

## Se désabonner

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Supprimer un abonnement supprime également les événements encore en file d’attente pour celui‑ci. Seuls les abonnements créés via l’API peuvent être supprimés de cette façon. Les webhooks du tableau de bord sont modifiés sur la page Webhooks.

## Charges utiles et signature

Les livraisons utilisent la même charge utile que les webhooks du tableau de bord (voir Structures de données) et sont signées avec le même schéma HMAC (voir Sécurité & Jetons API). Les abonnements API ne reçoivent jamais l’en‑tête hérité `token`, il faut donc vérifier l’en‑tête `X-FastComments-Signature` à la place.

## Répondre avec 410 Gone

Si le point de terminaison d’un abonnement API répond avec HTTP `410 Gone`, FastComments le considère comme une désinscription : l’abonnement est supprimé ainsi que ses événements en file d’attente, et aucune autre livraison n’est tentée. Les webhooks configurés dans le tableau de bord ne sont jamais supprimés automatiquement ; pour eux, un 410 est simplement un échec ordinaire. Tout autre statut d’échec est retenté et finit par désactiver le webhook, comme décrit dans Fonctionnement & Gestion des nouvelles tentatives.

## Tableau de bord

Les abonnements API sont listés sur la page Webhooks sous le domaine pour lequel ils ont été créés, où un administrateur peut les désactiver, les réactiver ou les supprimer.