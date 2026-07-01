## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| errorId | string | Nee |  |

## Respons

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRes, httpRes) = client.deleteEmailTemplateRenderError(
  tenantId = "my-tenant-123",
  id = "welcome-email",
  errorId = "render-err-456"
)

if apiRes.isSome:
  let empty = apiRes.get()
[inline-code-end]

---