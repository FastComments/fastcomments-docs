## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantBody | UpdateTenantBody | No |  |

## תגובה

מחזיר: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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