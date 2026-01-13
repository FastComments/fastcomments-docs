## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateTenantUserBody | UpdateTenantUserBody | 否 |  |
| updateComments | string | 否 |  |

## 回應

回傳: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'updateTenantUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
var body: UpdateTenantUserBody
body.email = "jane.doe@example.com"
body.displayName = "Jane Doe"
body.roles = @["moderator", "editor"]
body.isActive = true

let (response, httpResponse) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateTenantUserBody = body,
  updateComments = "Promoted user to moderator and editor roles"
)

if response.isSome:
  let flagResp = response.get()
  echo flagResp
else:
  echo "Update failed, status: ", httpResponse.status
[inline-code-end]