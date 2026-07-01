## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| search | string | No |  |
| options | GetGifsSearchOptions | No |  |

## Odgovor

Vraća: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## Primjer

[inline-code-attrs-start title = 'getGifsSearch Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetGifsSearchOptions(limit = 10, rating = "g")
let (responseOpt, httpResponse) = client.getGifsSearch(tenantId = "my-tenant-123", search = "funny cats", options = opts)
if responseOpt.isSome:
  let resp = responseOpt.get()
  # upotrijebi resp po potrebi
[inline-code-end]