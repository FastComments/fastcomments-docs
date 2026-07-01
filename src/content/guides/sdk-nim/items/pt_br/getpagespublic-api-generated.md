List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.  
Páginas que requerem SSO são filtradas de acordo com o acesso ao grupo do usuário solicitante.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetPagesPublicOptions | No |  |

## Response

Retorna: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Example

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]