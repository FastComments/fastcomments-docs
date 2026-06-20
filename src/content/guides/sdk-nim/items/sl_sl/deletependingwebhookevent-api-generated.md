## Parametri

| Name | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vrača: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'deletePendingWebhookEvent Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePendingWebhookEvent(tenantId = "my-tenant-123", id = "wh_evt_9f8b7a6c")
if response.isSome:
  let apiEmpty = response.get()
  echo "Pending webhook event deleted for tenant my-tenant-123"
else:
  echo "Failed to delete pending webhook event"
[inline-code-end]

---