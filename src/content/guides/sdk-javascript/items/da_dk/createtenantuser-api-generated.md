## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantUserBody | CreateTenantUserBody | Ja |  |

## Svar

Returnerer: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4b2a';
const digestFreq: DigestEmailFrequency = { interval: 'daily' };
const agentApprovalFreq: ImportedAgentApprovalNotificationFrequency = { mode: 'immediate' };
const createTenantUserBody: CreateTenantUserBody = {
  email: 'sara.martin@acme.co',
  name: 'Sara Martin',
  role: 'moderator',
  avatarUrl: 'https://cdn.acme.co/avatars/sara.jpg',
  notifyByEmail: true,                     // valgfrit parameter demonstreret
  digestEmailFrequency: digestFreq,
  importedAgentApprovalNotificationFrequency: agentApprovalFreq
};
const result: CreateTenantUserResponse = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---