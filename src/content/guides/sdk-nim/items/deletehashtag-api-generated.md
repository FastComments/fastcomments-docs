## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | No |  |
| tenantId | string | Yes |  |
| deleteHashTagRequest | DeleteHashTagRequest | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'deleteHashTag Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())

if response.isSome:
  let deleted = response.get()
  discard deleted
[inline-code-end]
