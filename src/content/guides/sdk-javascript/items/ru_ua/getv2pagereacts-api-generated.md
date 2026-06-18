## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Ответ

Возвращает: [`GetV2PageReacts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getV2PageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_82f4b3a9";
const urlId: string = "https://news.site.com/articles/2026/06/15/product-launch";
const response: GetV2PageReacts200Response = await getV2PageReacts(tenantId, urlId);
console.log(response);
[inline-code-end]

---