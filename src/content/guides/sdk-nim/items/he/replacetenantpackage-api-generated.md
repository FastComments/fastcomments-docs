## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | No |  |

## תשובה

מחזיר: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-replaceTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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