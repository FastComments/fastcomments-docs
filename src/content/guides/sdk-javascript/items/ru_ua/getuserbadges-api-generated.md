## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Нет |  |
| badgeId | string | Нет |  |
| type | number | Нет |  |
| displayedOnComments | boolean | Нет |  |
| limit | number | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'user_5f4d3c2a';
const badgeId: string = 'badge_top_contributor';
const type: number = 1;
const displayedOnComments: boolean = true;
const limit: number = 50;
const skip: number = 0;

const result: GetUserBadges200Response = await getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip);
[inline-code-end]