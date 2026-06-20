## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |
| sure | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenant(tenantId = "my-tenant-123", id = "", sure = "")
if response.isSome:
  let emptyResp = response.get()
else:
  discard httpResponse
[inline-code-end]

---