---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateTenantBody | UpdateTenantBody | Ні |  |

## Відповідь

Повертає: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenant(
  tenantId = "my-tenant-123",
  id = "tenant-456",
  updateTenantBody = UpdateTenantBody()
)
if response.isSome:
  let flagResponse = response.get()
  echo flagResponse
else:
  echo "No body returned; HTTP status: ", httpResponse.status
[inline-code-end]

---