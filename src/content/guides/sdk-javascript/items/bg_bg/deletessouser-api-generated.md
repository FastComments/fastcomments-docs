## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| deleteComments | boolean | Не |  |
| commentDeleteMode | string | Не |  |

## Отговор

Връща: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSSOUserAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за deleteSSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const userId: string = "sso-user-42";
const deleteComments: boolean = true;
const commentDeleteMode: string = "hard";

const detailedResult: DeleteSSOUserAPIResponse = await deleteSSOUser(
  tenantId,
  userId,
  deleteComments,
  commentDeleteMode
);

const simpleResult: DeleteSSOUserAPIResponse = await deleteSSOUser(
  tenantId,
  userId
);
[inline-code-end]