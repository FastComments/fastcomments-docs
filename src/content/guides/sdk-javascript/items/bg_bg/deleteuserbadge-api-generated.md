## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Отговор

Връща: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const badgeId: string = 'badge_9f8b2c1d';
const includeAudit: boolean | undefined = undefined; // незадължителен флаг (не се изисква от deleteUserBadge)
const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
[inline-code-end]