---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| id | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteV2PageReact'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteV2PageReact(tenantId = "my-tenant-123", urlId = "news/2026/politics-election", id = "react-456")
if response.isSome:
  let react = response.get()
  echo react
else:
  echo "No reaction returned, status: ", httpResponse.status
[inline-code-end]

---