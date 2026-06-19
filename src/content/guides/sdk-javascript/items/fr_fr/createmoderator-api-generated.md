## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createModeratorBody | CreateModeratorBody | Oui |  |

## Response

Renvoie : [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemple de createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // paramètre optionnel illustré
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---