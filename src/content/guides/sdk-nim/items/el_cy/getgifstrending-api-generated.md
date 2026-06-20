## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| locale | string | Όχι |  |
| rating | string | Όχι |  |
| page | float64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGifsTrending'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsTrending(tenantId = "my-tenant-123",
  locale = "en-US",
  rating = "pg-13",
  page = 1.0)
if response.isSome:
  let trending = response.get()
[inline-code-end]

---