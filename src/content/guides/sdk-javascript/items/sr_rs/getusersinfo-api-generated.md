Груписани подаци о корисницима за закупца. Дати userIds, враћају се подаци за приказ из User / SSOUser.  
Користи се у виџету за коментаре да обогати кориснике који су управо појавили путем догађаја присутности.  
Нема контекст странице: приватност се примењује уједначено (приватни профили су маскирани).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---