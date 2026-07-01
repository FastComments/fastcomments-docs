## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| deleteComments | boolean | Nie |  |
| commentDeleteMode | string | Nie |  |

## Odpowiedź

Zwraca: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSSOUserAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'deleteSSOUser Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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