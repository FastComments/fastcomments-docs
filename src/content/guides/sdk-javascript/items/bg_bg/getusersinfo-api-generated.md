Групова информация за потребител за наемател. При зададени userIds се връща информация за показване от User / SSOUser.  
Използва се от уиджета за коментари за обогатяване на потребители, които току‑що се появиха чрез събитие за наличност.  
Без контекст на страницата: поверителността се прилага еднообразно (частните профили се маскират).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Отговор

Връща: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]