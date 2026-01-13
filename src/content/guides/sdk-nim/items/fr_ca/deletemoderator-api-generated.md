## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| sendEmail | string | Non |  |

## Réponse

Renvoie: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerator(tenantId = "my-tenant-123", id = "moderator-456", sendEmail = "false")
if response.isSome:
  let flagResp = response.get()
  echo "Moderator deletion response: ", $flagResp
else:
  echo "No response body; HTTP status: ", $httpResponse.status
[inline-code-end]

---