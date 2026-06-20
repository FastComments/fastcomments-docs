## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createCommentParams | CreateCommentParams | Ne |  |
| isLive | bool | Ne |  |
| doSpamCheck | bool | Ne |  |
| sendEmails | bool | Ne |  |
| populateNotifications | bool | Ne |  |

## Odgovor

Vraća: [`Option[APISaveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_save_comment_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer upotrebe saveComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---