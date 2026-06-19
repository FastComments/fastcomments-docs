Масовне информације о корисницима за закупца. За дате userIds, враћа информације за приказ из User / SSOUser.
Користи га widget коментара да обогати кориснике који су се управо појавили путем presence догађаја.
Без контекста странице: приватност се примењује уједначено (приватни профили су замагљени).

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo захтева само tenantId и ids; опциони параметри овде нису применљиви.
[inline-code-end]

---