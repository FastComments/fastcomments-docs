## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |

## Respons

Retourneert: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getPages Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPages(tenantId = "news-site-456")
if response.isSome:
  let pages = response.get()
  echo "Received pages response: ", pages
else:
  echo "No pages returned. HTTP response: ", httpResponse
[inline-code-end]