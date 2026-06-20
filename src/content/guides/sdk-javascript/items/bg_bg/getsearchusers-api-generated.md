## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| value | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getSearchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const searchValue: string = 'jane.doe@acme-corp.com';
  const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const responseWithSso: ModerationUserSearchResponse = await getSearchUsers(searchValue, ssoToken);
  const searchValue2: string = 'michael.brown';
  const responseWithoutSso: ModerationUserSearchResponse = await getSearchUsers(searchValue2);
  console.log(responseWithSso, responseWithoutSso);
})();
[inline-code-end]

---