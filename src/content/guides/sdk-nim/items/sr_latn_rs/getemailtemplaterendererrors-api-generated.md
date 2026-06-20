## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getEmailTemplateRenderErrors'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateRenderErrors(tenantId = "my-tenant-123", id = "", skip = 0.0)
if response.isSome:
  let templateErrors = response.get()
  discard templateErrors
else:
  discard httpResponse
[inline-code-end]

---