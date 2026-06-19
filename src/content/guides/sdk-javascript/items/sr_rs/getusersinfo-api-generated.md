Пакетни подаци о корисницима за tenant. На основу userIds, враћа приказне информације из User / SSOUser.
Користи га comment widget да обогати кориснике који су управо појавили путем presence event-а.
Нема page context-а: приватност се спроводи уједначено (приватни профили су сакривени).

## Параметри

| Име | Тип | Обавезно | Опис |
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
// getUsersInfo захтева само tenantId и ids; опциони параметри овде се не примењују.
[inline-code-end]

---