## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| redirectURL | string | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer sendLoginLink'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "fastcomments-tenant-42", id = "user-9876", redirectURL = "https://news.example.com/articles/2026/fastcomments-login")
if response.isSome:
  let loginResp = response.get()
  echo "Login link sent successfully"
else:
  echo "Failed to send login link"
[inline-code-end]

---