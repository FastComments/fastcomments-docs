## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| externalId | string | Nej |  |
| eventType | string | Nej |  |
| domain | string | Nej |  |
| attemptCountGT | float64 | Nej |  |

## Svar

Returnerer: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-456abc",
  externalId = "ext-7890",
  eventType = "comment_created",
  domain = "news.example.com",
  attemptCountGT = 2.0
)

if response.isSome:
  let pending = response.get()
  echo pending
else:
  echo "No pending webhook event count returned; HTTP status: ", httpResponse.status
[inline-code-end]

---