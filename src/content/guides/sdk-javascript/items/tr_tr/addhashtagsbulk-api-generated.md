## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Yanıt

Döndürür: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Örnek

[inline-code-attrs-start title = 'addHashTagsBulk Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Kiracı tanımlayıcısı oluştur (isteğe bağlı parametre)
const tenantId: string = "tenant_9f8c2b7a";

// Bireysel etiket girdilerini hazırla
const tag1: BulkCreateHashTagsBodyTagsInner = {
  name: "product-feedback",
  label: "Product Feedback",
  color: "#1f8a70",
  description: "User suggestions and enhancement requests",
  isActive: true
};

const tag2: BulkCreateHashTagsBodyTagsInner = {
  name: "bug-report",
  label: "Bug Report",
  color: "#d64545",
  description: "User-reported defects and issues",
  isActive: true
};

// Toplu oluşturma gövdesi (isteğe bağlı parametre)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Global eşzamansız fonksiyonu çağır ve tiplenmiş sonucu ata
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]