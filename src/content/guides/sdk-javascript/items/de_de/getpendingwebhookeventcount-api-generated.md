## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Nein |  |
| externalId | string | Nein |  |
| eventType | string | Nein |  |
| type | string | Nein |  |
| domain | string | Nein |  |
| attemptCountGT | number | Nein |  |

## Antwort

Gibt zurück: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d3b7a2f";
const commentId: string | undefined = "comment_79a2b";
const eventType: string | undefined = "comment.created";
const domain: string | undefined = "forum.acme-corp.com";
const attemptCountGT: number | undefined = 1;
const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined,
  eventType,
  undefined,
  domain,
  attemptCountGT
);
[inline-code-end]

---