## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | seq[CreateCommentParams] | No |  |
| options | SaveCommentsBulkOptions): (Option[seq[SaveCommentsBulkResponse]] | No |  |
| id | string | No |  |
| fromName | string | No |  |

## Yanıt

Döndürür: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Örnek

[inline-code-attrs-start title = 'saveCommentsBulk Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = @[],
  options = SaveCommentsBulkOptions(),
  id = "",
  fromName = ""
)

if response.isSome:
  let result = response.get()
[inline-code-end]