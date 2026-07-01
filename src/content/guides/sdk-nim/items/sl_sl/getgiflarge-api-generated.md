## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| largeInternalURLSanitized | string | No |  |

## Odgovor

Vrne: [`Option[GifGetLargeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_gif_get_large_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getGifLarge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (gifOpt, httpResp) = client.getGifLarge(
  tenantId = "my-tenant-123",
  largeInternalURLSanitized = "https://cdn.example.com/gifs/large123.gif")
if gifOpt.isSome:
  let gif = gifOpt.get()
  echo gif
[inline-code-end]