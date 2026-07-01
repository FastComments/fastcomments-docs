## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteV1PageReact Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (reactOpt, httpResp) = client.deleteV1PageReact(tenantId = "my-tenant-123", urlId = "news/article-title")
if reactOpt.isSome:
  let react = reactOpt.get()
[inline-code-end]