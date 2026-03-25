## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| createModeratorBody | CreateModeratorBody | Ja |  |

## Respons

Geeft terug: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createModerator Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7d9f2b4a';
const customConfig: CustomConfigParameters = { timezone: 'UTC', moderationQueueEnabled: true };
const createModeratorBody: CreateModeratorBody = {
  email: 'jane.martin@publisher.com',
  displayName: 'Jane Martin',
  roles: ['moderator'],
  sendWelcomeEmail: true,
  customConfig
};
const response: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---