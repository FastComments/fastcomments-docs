## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Response

Retourneert: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Example

[inline-code-attrs-start title = 'flagComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(
  tenantId = "my-tenant-123",
  id = "cmt-98765",
  userId = "user-12345",
  anonUserId = ""
)

if response.isSome:
  let flagResp = response.get()
  echo "Flag response received"
else:
  echo "No flag response returned"
[inline-code-end]

---