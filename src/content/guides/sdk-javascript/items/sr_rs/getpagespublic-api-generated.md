Листа страница за tenant. Користи се од стране FChat десктоп клијента да попуни његову листу соба. Захтева да `enableFChat` буде true на решеном прилагођеном конфигу за сваку страницу. Странице које захтевају SSO се филтрирају према приступу групе корисника који је послао захтев.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | number | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | boolean | Не |  |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---