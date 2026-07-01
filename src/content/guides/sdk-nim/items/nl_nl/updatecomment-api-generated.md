## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| updatableCommentParams | UpdatableCommentParams | Nee |  |
| options | UpdateCommentOptions | Nee |  |

## Respons

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'updateComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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