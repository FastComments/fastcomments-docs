## Параметри

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Одговор

Враћа: [`CreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadgeResponse.ts)

## Пример

[inline-code-attrs-start title = 'createUserBadge Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "company-42";

const badgeParams: CreateUserBadgeParams = {
  name: "Community Champion",
  iconUrl: "https://assets.example.com/badges/champion.png",
  // opis je opciona i ovde je izostavljen
};

const result: CreateUserBadgeResponse = await createUserBadge(tenantId, badgeParams);
[inline-code-end]