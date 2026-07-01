## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | No |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | No |  |

## Odpowiedź

Zwraca: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Przykład

[inline-code-attrs-start title = 'replaceTenantPackage Przykład'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.replaceTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-456",
  replaceTenantPackageBody = ReplaceTenantPackageBody()
)
if optResp.isSome:
  let resp = optResp.get()
  discard resp
[inline-code-end]