## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| updatableCommentParams | UpdatableCommentParams | No |  |
| options | UpdateCommentOptions | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentParams = UpdatableCommentParams(content: "Updated comment content")
let updateOpts = UpdateCommentOptions(force: false)

let (respOpt, httpResp) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  updatableCommentParams = commentParams,
  options = updateOpts
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]