List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

Kiralayan bir tenant için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.  
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.  
SSO gerektiren sayfalar, talep eden kullanıcının grup erişimine göre filtrelenir.

## Parameters

| Ad | Tür | Gereklidir | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| options | GetPagesPublicOptions | Hayır |  |

## Response

Döndürür: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Example

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]