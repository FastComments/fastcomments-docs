## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Да |  |
| updateComments | boolean | Нет |  |

## Ответ

Возвращает: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример putSSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-42';
const id: string = 'usr-73a1b2';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: 'marcus.ingram@acme.com',
  givenName: 'Marcus',
  familyName: 'Ingram',
  roles: ['editor', 'project_owner'],
  enabled: true
};
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, id, updateAPISSOUserData, true);
[inline-code-end]

---