## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| createModeratorBody | CreateModeratorBody | Sì |  |

## Risposta

Restituisce: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // parametro opzionale dimostrato
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---