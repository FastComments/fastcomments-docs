## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantPackage(tenantId = "my-tenant-123", id = "pkg-premium", updateTenantPackageBody = UpdateTenantPackageBody())
if response.isSome:
  let updated = response.get()
  echo "Updated package received:", updated
else:
  echo "Update failed, HTTP status: ", httpResponse.status
[inline-code-end]