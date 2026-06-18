Перелік сторінок для орендаря. Використовується десктоп-клієнтом FChat для заповнення його списку кімнат.
Вимагає, щоб `enableFChat` було встановлено в true у вирішеній кастомній конфігурації для кожної сторінки.
Сторінки, які вимагають SSO, фільтруються відповідно до групового доступу користувача, що виконує запит.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| cursor | string | Ні |  |
| limit | number | Ні |  |
| q | string | Ні |  |
| sortBy | PagesSortBy | Ні |  |
| hasComments | boolean | Ні |  |

## Відповідь

Повертає: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---