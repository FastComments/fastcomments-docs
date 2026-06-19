---
Перелічує сторінки для орендаря. Використовується настільним клієнтом FChat для заповнення списку кімнат.
Потребує, щоб `enableFChat` було встановлено в true у вирішеній користувацькій конфігурації для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються відповідно до групового доступу користувача, що робить запит.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| cursor | string | Ні |  |
| limit | number | Ні |  |
| q | string | Ні |  |
| sortBy | PagesSortBy | Ні |  |
| hasComments | boolean | Ні |  |

## Response

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Example

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---