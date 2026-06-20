## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| createCommentParams | CreateCommentParams | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| sendEmails | bool | No |  |
| populateNotifications | bool | No |  |

## Response

Restituisce: [`Option[APISaveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_save_comment_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di saveComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createCommentParams = CreateCommentParams(
  urlId = "news/2026/major-policy-change",
  content = "This is a thoughtful comment on the policy change and its potential impacts.",
  authorName = "Morgan Lee",
  authorEmail = "morgan.lee@example.org",
  tags = @["policy","analysis"],
  extraData = @[])

let (response, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = createCommentParams,
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true)

if response.isSome:
  let saved = response.get()
  discard saved
[inline-code-end]