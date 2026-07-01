## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| namespace | string | Nie |  |
| component | string | Nie |  |
| options | GetTranslationsOptions | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetTranslationsOptions()
let (maybeResp, httpResp) = client.getTranslations(namespace = "my-tenant-123", component = "news/article-title", options = opts)
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]