Commentaires récents dans exactement la forme utilisée par les livraisons de webhook, pour créer des intégrations (par exemple les données d'exemple Zapier). Chaque événement délivre le même objet commentaire, de sorte que `event` n'a besoin d'être que valide.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| event | WebhookEventName | No |  |
| limit | number | No |  |

## Réponse

Retourne : [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const responseAll: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created",
  10
);

const responseWithEvent: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created"
);

const responseBasic: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId
);
[inline-code-end]