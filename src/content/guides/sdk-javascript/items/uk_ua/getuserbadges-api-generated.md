## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| badgeId | string | Ні |  |
| type | number | Ні |  |
| displayedOnComments | boolean | Ні |  |
| limit | number | Ні |  |
| skip | number | Ні |  |

## Відповідь

Повертає: [`GetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgesResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant-01";
  const userId: string = "user-42";
  const badgeId: string = "badge-gold";
  const type: number = 1;
  const displayedOnComments: boolean = true;
  const limit: number = 10;
  const skip: number = 5;

  const fullResult: GetUserBadgesResponse = await getUserBadges(
    tenantId,
    userId,
    badgeId,
    type,
    displayedOnComments,
    limit,
    skip
  );

  const minimalResult: GetUserBadgesResponse = await getUserBadges(tenantId);
}
example();
[inline-code-end]

---