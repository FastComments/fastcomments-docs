## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| sure | string | Ne |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primjer

[inline-code-attrs-start title = 'deleteTenant Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenant(tenantId = "my-tenant-123", id = "", sure = "")
if response.isSome:
  let emptyResp = response.get()
else:
  discard httpResponse
[inline-code-end]

---