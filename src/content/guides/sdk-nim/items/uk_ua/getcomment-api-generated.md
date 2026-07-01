## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Відповідь

Повертає: [`Option[APIGetCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comment_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-789")
if response.isSome:
  let comment = response.get()
  discard comment
[inline-code-end]

---