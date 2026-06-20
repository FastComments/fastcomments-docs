## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |
| skip | float64 | Nej |  |

## Svar

Returnerer: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateRenderErrors(tenantId = "my-tenant-123", id = "", skip = 0.0)
if response.isSome:
  let templateErrors = response.get()
  discard templateErrors
else:
  discard httpResponse
[inline-code-end]

---