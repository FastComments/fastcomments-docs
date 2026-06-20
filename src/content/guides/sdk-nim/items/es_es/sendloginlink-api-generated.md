---
## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| redirectURL | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de sendLoginLink'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "my-tenant-123", id = "user-456", redirectURL = "https://app.newsportal.com/welcome")
if response.isSome:
  let apiResp = response.get()
  echo "Login link sent successfully"
else:
  echo "Failed to send login link, HTTP status: ", $httpResponse.status
[inline-code-end]

---