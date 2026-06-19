Списък на страниците за наемател. Използва се от десктоп клиента FChat за попълване на списъка с чат стаи.
Изисква `enableFChat` да бъде true в разрешената персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират спрямо груповия достъп на потребителя, който прави заявката.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | number | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | boolean | Не |  |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---