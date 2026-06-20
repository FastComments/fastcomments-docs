## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| largeInternalURLSanitized | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GifGetLargeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_gif_get_large_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGifLarge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifLarge(tenantId = "news-tenant-42", largeInternalURLSanitized = "")
if response.isSome:
  let gif = response.get()
  echo "Received GifGetLargeResponse"
  discard gif
else:
  echo "No gif returned, HTTP status: " & $httpResponse.status
[inline-code-end]

---