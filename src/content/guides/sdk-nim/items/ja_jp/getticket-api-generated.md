## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| userId | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetTicket_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket200response.nim)

## 例

[inline-code-attrs-start title = 'getTicket の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "", userId = "")
if response.isSome:
  let ticket = response.get()
  discard ticket
[inline-code-end]

---