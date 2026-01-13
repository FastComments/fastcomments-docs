## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetEmailTemplates_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates200response.nim)

## Primjer

[inline-code-attrs-start title = 'getEmailTemplates Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let templates = response.get()
  echo templates
else:
  echo "No templates returned"
[inline-code-end]

---