## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| value | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getSearchSites'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSites(value = "news/2026-olympics", sso = "sso-user-9876-token")
if response.isSome:
  let searchResponse: ModerationSiteSearchResponse = response.get()
  echo "Found sites for search:", searchResponse
[inline-code-end]

---