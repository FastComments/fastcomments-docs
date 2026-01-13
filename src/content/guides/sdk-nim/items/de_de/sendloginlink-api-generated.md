## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| redirectURL | string | Nein |  |

## Antwort

Gibt zurück: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'sendLoginLink Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "fastcomments-tenant-42", id = "user-9876", redirectURL = "https://news.example.com/articles/2026/fastcomments-login")
if response.isSome:
  let loginResp = response.get()
  echo "Login link sent successfully"
else:
  echo "Failed to send login link"
[inline-code-end]

---