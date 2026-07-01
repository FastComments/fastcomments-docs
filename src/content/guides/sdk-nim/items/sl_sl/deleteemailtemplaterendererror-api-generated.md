## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| errorId | string | No |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRes, httpRes) = client.deleteEmailTemplateRenderError(
  tenantId = "my-tenant-123",
  id = "welcome-email",
  errorId = "render-err-456"
)

if apiRes.isSome:
  let empty = apiRes.get()
[inline-code-end]