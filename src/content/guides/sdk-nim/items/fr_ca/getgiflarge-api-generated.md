## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| largeInternalURLSanitized | string | Non |  |

## Réponse

Renvoie : [`Option[GifGetLargeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_gif_get_large_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGifLarge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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