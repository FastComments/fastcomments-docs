## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| includeEmail | bool | Nein |  |
| includeIP | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[ModerationAPICommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_comment_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getModerationComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerationComment(commentId = "cmt-8f3b2a9d", includeEmail = false, includeIP = false, sso = "")
if response.isSome:
  let comment = response.get()
  discard comment
else:
  echo "No moderation comment returned"
[inline-code-end]