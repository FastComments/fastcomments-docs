## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |
| skip | float64 | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getEmailTemplateRenderErrors'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getEmailTemplateRenderErrors(
  tenantId = "my-tenant-123",
  id = "welcome-template",
  skip = 0.0
)

if optResp.isSome:
  let resp = optResp.get()
  # χρησιμοποιήστε resp όπως χρειάζεται
else:
  # χειριστείτε την απουσία απάντησης
  discard
[inline-code-end]