## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantPackageBody | UpdateTenantPackageBody | No |  |

## Respons

Retourneert: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'updateTenantPackage Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantPackage(tenantId = "my-tenant-123", id = "pkg-premium", updateTenantPackageBody = UpdateTenantPackageBody())
if response.isSome:
  let updated = response.get()
  echo "Updated package received:", updated
else:
  echo "Update failed, HTTP status: ", httpResponse.status
[inline-code-end]

---