## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 否 |  |

## 回應

回傳: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'replaceTenantPackage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let replaceBody = ReplaceTenantPackageBody(
  packageName = "Community Pro",
  seats = 500,
  enableModeration = true,
  features = @["moderation", "analytics", "single-sign-on"]
)

let (response, httpResponse) = client.replaceTenantPackage(
  tenantId = "my-tenant-123",
  id = "community-pro",
  replaceTenantPackageBody = replaceBody
)

if response.isSome:
  let flagResp = response.get()
  echo "Package replaced for tenant: ", "my-tenant-123"
  discard flagResp
[inline-code-end]

---