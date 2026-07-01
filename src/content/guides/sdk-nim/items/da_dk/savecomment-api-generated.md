## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | CreateCommentParams | Nej |  |
| options | SaveCommentOptions | Nej |  |

## Svar

Returnerer: [`Option[APISaveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_save_comment_response.nim)

## Eksempel

[inline-code-attrs-start title = 'saveComment Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentParams = CreateCommentParams(
  body: "Great read, thanks!",
  name: "Alice Smith",
  email: "alice@example.com",
  parentId: ""
)

let saveOpts = SaveCommentOptions(
  isPreview: false,
  isApproved: true,
  skipSpamCheck: false
)

let (apiResponse, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = commentParams,
  options = saveOpts
)

if apiResponse.isSome:
  let saved = apiResponse.get()
  echo saved
[inline-code-end]