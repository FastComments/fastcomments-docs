Webhooks peuvent également être gérés via l'API REST. C’est ainsi que des intégrations comme Zapier s’abonnent aux événements de commentaires sans toucher au tableau de bord, et cela suit le modèle REST Hooks : s’abonner, recevoir des événements, se désabonner.

Les abonnements API coexistent avec les webhooks configurés dans le tableau de bord. Un événement de commentaire est livré à chaque webhook dont le domaine correspond, chacun sous forme de livraison distincte, quel que soit le mode de création du webhook.

## Authentification

Chaque requête doit inclure votre clé API dans l’en-tête `x-api-key` (ou le paramètre de requête `API_KEY`) ainsi que votre ID de locataire dans le paramètre de requête `tenantId`. Les deux sont affichés sur la page API Secret du tableau de bord.

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
| `url` | Yes | Une URL http ou https absolue. |
| `event` | Yes | `comment-created`, `comment-updated` ou `comment-deleted`. |
| `domain` | No | Un domaine provenant de la configuration de votre compte. La valeur par défaut est `*`, qui reçoit les événements pour tous les domaines. |
| `method` | No | `POST` (par défaut), `PUT` ou `DELETE`. |

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

S’abonner à la même URL pour le même événement et le même domaine renvoie à nouveau l’abonnement existant au lieu de créer un doublon, ce qui permet à un client de réessayer en toute sécurité. Chaque locataire peut avoir jusqu’à 50 abonnements API.

## Lister

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Renvoie tous les webhooks du locataire, y compris ceux gérés dans le tableau de bord (`"source": "dashboard"`). Filtrez avec `event`, `domain` ou `source`.

## Se désabonner

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Supprimer un abonnement supprime également tous les événements encore en file d’attente pour celui‑ci. Seuls les abonnements créés via l’API peuvent être supprimés de cette manière ; un webhook du tableau de bord, ou un identifiant qui n’existe pas sur votre compte, renvoie `404` avec le code `not-found`. Les webhooks du tableau de bord sont modifiés sur la page Webhooks.

## Charges utiles et signature

Les livraisons utilisent la même charge utile que les webhooks du tableau de bord (voir Structures de données) et sont signées avec le même schéma HMAC (voir Sécurité & jetons API). Les abonnements API ne reçoivent jamais l’en‑tête hérité `token`, il faut donc vérifier l’en‑tête `X-FastComments-Signature` à la place.

## Exemples de charges utiles

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Renvoie les commentaires les plus récents du compte exactement dans la forme qu’une livraison transporte, afin qu’une intégration puisse afficher de vraies données d’exemple avant l’arrivée du premier événement. `event` est optionnel et uniquement validé, puisque chaque événement délivre le même objet commentaire. `limit` vaut par défaut 3 et accepte de 1 à 10. Coûte 2 crédits API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Répondre avec 410 Gone

Si le point de terminaison d’un abonnement API répond avec HTTP `410 Gone`, FastComments considère cela comme un désabonnement : l’abonnement est supprimé ainsi que ses événements en file d’attente, et aucune autre livraison n’est tentée. Les webhooks configurés dans le tableau de bord ne sont jamais supprimés automatiquement ; pour eux, un 410 représente simplement un échec. Tout autre statut d’échec est réessayé et finit par désactiver le webhook, comme décrit dans Fonctionnement & Gestion des nouvelles tentatives.

## Tableau de bord

Les abonnements API apparaissent dans la liste des Webhooks avec la source **API**, où un administrateur peut les modifier, les désactiver, les réactiver ou les supprimer.

---