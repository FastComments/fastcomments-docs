## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteTenantPackage Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponse, httpResponse) = client.deleteTenantPackage(tenantId = "my-tenant-123", id = "pkg-456def")
if apiResponse.isSome:
  let emptyResponse = apiResponse.get()
[inline-code-end]