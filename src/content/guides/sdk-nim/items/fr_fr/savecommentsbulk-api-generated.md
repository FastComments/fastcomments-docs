## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | seq[CreateCommentParams] | Non |  |
| options | SaveCommentsBulkOptions): (Option[seq[SaveCommentsBulkResponse]] | Non |  |
| id | string | Non |  |
| fromName | string | Non |  |

## Réponse

Retourne : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'saveCommentsBulk Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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