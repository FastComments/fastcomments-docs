Масова информация за потребители за наемател. При зададени userIds връща информация за показване от User / SSOUser.
Използва се от компонента за коментари за обогатяване на потребители, които току-що се появиха чрез събитие за присъствие.
Няма контекст на страница: поверителността се прилага еднакво (личните профили са маскирани).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo изисква само tenantId и ids; незадължителните параметри не са приложими тук.
[inline-code-end]