## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createModeratorBody | CreateModeratorBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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