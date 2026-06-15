---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateTenantUserBody | UpdateTenantUserBody | Oui |  |
| updateComments | string | Non |  |

## Réponse

Renvoie : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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