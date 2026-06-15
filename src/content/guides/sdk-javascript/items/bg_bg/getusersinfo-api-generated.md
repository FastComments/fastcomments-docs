Масова информация за потребители за наемател. При предоставени userIds връща информация за показване от User / SSOUser.
Използва се от коментарния widget за обогатяване на потребители, които току-що се появиха чрез събитие за присъствие.
Няма контекст на страница: поверителността се прилага еднакво (частните профили са маскирани).

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Отговор

Връща: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // незадължително; ако е undefined, по подразбиране запетая
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]