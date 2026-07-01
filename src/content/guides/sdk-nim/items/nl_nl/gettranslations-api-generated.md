## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| namespace | string | Nee |  |
| component | string | Nee |  |
| options | GetTranslationsOptions | Nee |  |

## Respons

Retourneert: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTranslations Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetTranslationsOptions()
let (maybeResp, httpResp) = client.getTranslations(namespace = "my-tenant-123", component = "news/article-title", options = opts)
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]