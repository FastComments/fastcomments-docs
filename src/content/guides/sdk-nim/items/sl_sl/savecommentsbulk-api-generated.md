## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createCommentParams | seq[CreateCommentParams] | Ne |  |
| options | SaveCommentsBulkOptions): (Option[seq[SaveCommentsBulkResponse]] | Ne |  |
| id | string | Ne |  |
| fromName | string | Ne |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'saveCommentsBulk Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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