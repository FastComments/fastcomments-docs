## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Ne |  |
| tenantId | string | Da |  |
| deleteHashTagRequest | DeleteHashTagRequest | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---