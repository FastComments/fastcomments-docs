## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | CreateCommentParams | Nej |  |
| isLive | bool | Nej |  |
| doSpamCheck | bool | Nej |  |
| sendEmails | bool | Nej |  |
| populateNotifications | bool | Nej |  |

## Svar

Returnerer: [`Option[SaveComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comment200response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på saveComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = CreateCommentParams(
    content = "This article was really helpful, thanks!",
    urlId = "news/2025-11/ai-regulations",
    authorName = "Jane Doe",
    authorEmail = "jane.doe@example.com",
    tags = @["policy", "analysis"]
  ),
  isLive = true,
  doSpamCheck = true,
  sendEmails = true,
  populateNotifications = false
)

if response.isSome:
  let saved = response.get()
  discard saved
[inline-code-end]

---