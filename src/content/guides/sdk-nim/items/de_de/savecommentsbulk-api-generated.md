## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| createCommentParams | seq[CreateCommentParams] | Nein |  |
| options | SaveCommentsBulkOptions): (Option[seq[SaveCommentsBulkResponse]] | Nein |  |
| id | string | Nein |  |
| fromName | string | Nein |  |

## Antwort

Rückgabe: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'saveCommentsBulk Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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