## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Hayır |  |

## Yanıt

Döndürür: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Örnek

[inline-code-attrs-start title = 'replaceTenantPackage Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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