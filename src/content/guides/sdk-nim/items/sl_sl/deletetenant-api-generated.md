## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| sure | string = "" | No |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteTenant Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteTenant(tenantId = "my-tenant-123", id = "tenant-to-delete", sure = "yes")
if respOpt.isSome:
  let emptyResp = respOpt.get()
[inline-code-end]

---