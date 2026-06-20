## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| urlId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de putReopenThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putReopenThread(urlId = "news/2026-election-analysis", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Reopen succeeded, response: ", apiResp
else:
  echo "Reopen failed, HTTP status: ", httpResponse.status
[inline-code-end]

---