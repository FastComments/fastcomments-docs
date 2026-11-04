Bulk информация за потребители на даден tenant. При подадени userIds връща информация за показване от User / SSOUser.
Използва се от коментарния уиджет за обогатяване на потребители, които току-що се появиха чрез събитие за присъствие.
Без контекст на страница: поверителността се прилага еднакво (личните профили се маскират).

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]