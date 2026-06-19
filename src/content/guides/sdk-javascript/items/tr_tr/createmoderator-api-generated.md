## Parametreler

| Name | Type | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createModeratorBody | CreateModeratorBody | Evet |  |

## Yanıt

Döndürür: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createModerator Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // isteğe bağlı parametre örneği
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]