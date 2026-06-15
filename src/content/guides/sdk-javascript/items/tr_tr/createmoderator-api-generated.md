## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createModeratorBody | CreateModeratorBody | Evet |  |

## Yanıt

Döndürür: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createModerator Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b21';
const createModeratorBody: CreateModeratorBody = {
  moderator: {
    name: 'Alex Rivera',
    email: 'alex.rivera@fastcomments.io',
    role: 'global_moderator',
    enabled: true,
  },
  // isteğe bağlı parametreler gösteriliyor:
  notifyUser: true,
  permissions: ['delete_comment', 'edit_comment', 'ban_user'],
  customConfig: { dashboardTheme: 'dark' } as unknown as CustomConfigParameters
};
const result: CreateModerator200Response = await createModerator(tenantId, createModeratorBody);
[inline-code-end]