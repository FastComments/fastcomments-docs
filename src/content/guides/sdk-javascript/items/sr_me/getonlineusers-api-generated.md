Тренутно-онлајн гледаоци странице: особе чија websocket сесија је претплаћена на страницу управо сада.
Враћа anonCount + totalCount (сви претплатници у соби, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## Пример

[inline-code-attrs-start title = 'getOnlineUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---