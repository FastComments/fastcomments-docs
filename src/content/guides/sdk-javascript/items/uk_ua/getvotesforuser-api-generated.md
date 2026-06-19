## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |

## Відповідь

Повертає: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getVotesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "northwind-io";
const urlId: string = "blog/2025/06/typescript-updates";
const userId: string = "u_72f9b1c4";
const anonUserId: string = "anon_e4d2a9";

const votesForRegistered: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, userId);
const votesForAnonymous: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
[inline-code-end]

---