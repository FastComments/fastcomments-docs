List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Bir kiracı için sayfaları listele. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.

Requires `enableFChat` to be true on the resolved custom config for each page.  
Her sayfa için çözülen özel yapılandırmada `enableFChat` değerinin true olması gerekir.

Pages that require SSO are filtered against the requesting user's group access.  
SSO gerektiren sayfalar, istek yapan kullanıcının grup erişimine göre filtrelenir.

## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| cursor | string | Hayır |  |
| limit | number | Hayır |  |
| q | string | Hayır |  |
| sortBy | PagesSortBy | Hayır |  |
| hasComments | boolean | Hayır |  |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Example

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]