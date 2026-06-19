## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Yanıt

Döndürür: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createTenantUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4b2a';
const digestFreq: DigestEmailFrequency = { interval: 'daily' };
const agentApprovalFreq: ImportedAgentApprovalNotificationFrequency = { mode: 'immediate' };
const createTenantUserBody: CreateTenantUserBody = {
  email: 'sara.martin@acme.co',
  name: 'Sara Martin',
  role: 'moderator',
  avatarUrl: 'https://cdn.acme.co/avatars/sara.jpg',
  notifyByEmail: true,                     // isteğe bağlı parametre gösterimi
  digestEmailFrequency: digestFreq,
  importedAgentApprovalNotificationFrequency: agentApprovalFreq
};
const result: CreateTenantUserResponse = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]