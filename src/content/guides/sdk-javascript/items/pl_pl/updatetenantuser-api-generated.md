## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updateTenantUserBody | UpdateTenantUserBody | Tak |  |
| updateComments | string | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const userId: string = 'u_72b9f4';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['moderator'],
  suspended: false
};
const updateComments: string = 'Promoted to moderator after review of activity and community feedback';
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, userId, updateTenantUserBody, updateComments);
[inline-code-end]

---