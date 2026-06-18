Масовне информације о корисницима за tenant. За дате userIds, враћа информације за приказ из User / SSOUser.
Користи се у видџету за коментаре да обогати кориснике који су се управо појавили путем presence event-а.
Нема контекста странице: приватност се примењује једнако (приватни профили су маскирани).

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Респонс

Враћа: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Пример

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // опционално; ако је undefined, подразумевано је зарез
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---