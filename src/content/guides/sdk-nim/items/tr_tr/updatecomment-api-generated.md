## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| updatableCommentParams | UpdatableCommentParams | Hayır |  |
| contextUserId | string | Hayır |  |
| doSpamCheck | bool | Hayır |  |
| isLive | bool | Hayır |  |

## Yanıt

Döndürür: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Örnek

[inline-code-attrs-start title = 'updateComment Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654",
  updatableCommentParams = UpdatableCommentParams(
    text = "Updated comment: corrected facts and clarified wording.",
    isApproved = true,
    tags = @["news", "update"]
  ),
  contextUserId = "user-456",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let apiResp = response.get()
  discard apiResp
[inline-code-end]

---