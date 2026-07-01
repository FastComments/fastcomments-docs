## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |
| errorId | string | Nej |  |

## Svar

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRes, httpRes) = client.deleteEmailTemplateRenderError(
  tenantId = "my-tenant-123",
  id = "welcome-email",
  errorId = "render-err-456"
)

if apiRes.isSome:
  let empty = apiRes.get()
[inline-code-end]