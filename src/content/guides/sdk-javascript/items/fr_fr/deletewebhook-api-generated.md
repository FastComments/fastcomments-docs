Désabonnements (REST hook unsubscribe). Seules les souscriptions créées via cette API peuvent être
supprimées ici ; les webhooks gérés depuis le tableau de bord sont modifiés dans le tableau de bord.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const webhookId: string = "wh_98765";

const result: APIEmptyResponse = await deleteWebhook(tenantId, webhookId);
[inline-code-end]

---