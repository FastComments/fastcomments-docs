---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| largeInternalURLSanitized | string | Да |  |

## Отговор

Връща: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_marketing_tenant_7';
const largeInternalURLSanitized: string = 'https://cdn.acmeinc.com/gifs/promo-spring-2026_large_sanitized.gif';
const includePreview: boolean | undefined = undefined; // незадължителен флаг, който повикващият може да използва
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
console.log(result, includePreview);
[inline-code-end]

---