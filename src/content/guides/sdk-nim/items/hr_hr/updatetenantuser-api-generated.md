## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateTenantUserBody | UpdateTenantUserBody | Ne |  |
| updateComments | string | Ne |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primjer

[inline-code-attrs-start title = 'updateTenantUser Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-987",
  updateTenantUserBody = UpdateTenantUserBody(
    displayName = "Jane Doe",
    email = "jane.doe@example.com",
    roles = @["moderator", "editor"],
    isActive = true
  ),
  updateComments = "true"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---