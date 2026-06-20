## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## 例

[inline-code-attrs-start title = 'getModerators の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Received moderators response:", moderators
else:
  echo "No moderators returned"
[inline-code-end]

---