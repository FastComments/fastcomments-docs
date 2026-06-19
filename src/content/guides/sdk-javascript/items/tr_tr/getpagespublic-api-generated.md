Bir kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenmiş özel yapılandırmada `enableFChat`'in true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| cursor | string | Hayır |  |
| limit | number | Hayır |  |
| q | string | Hayır |  |
| sortBy | PagesSortBy | Hayır |  |
| hasComments | boolean | Hayır |  |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---