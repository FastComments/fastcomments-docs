## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

回傳: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'deletePendingWebhookEvent 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePendingWebhookEvent(tenantId = "my-tenant-123", id = "wh_evt_9f8b7a6c")
if response.isSome:
  let apiEmpty = response.get()
  echo "Pending webhook event deleted for tenant my-tenant-123"
else:
  echo "Failed to delete pending webhook event"
[inline-code-end]

---