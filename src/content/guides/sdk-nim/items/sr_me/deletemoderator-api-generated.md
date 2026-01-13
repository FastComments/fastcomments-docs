## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| sendEmail | string | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer za deleteModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerator(tenantId = "my-tenant-123", id = "moderator-456", sendEmail = "false")
if response.isSome:
  let flagResp = response.get()
  echo "Moderator deletion response: ", $flagResp
else:
  echo "No response body; HTTP status: ", $httpResponse.status
[inline-code-end]