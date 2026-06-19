## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantUserBody | CreateTenantUserBody | Да |  |

## Отговор

Връща: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4b2a';
const digestFreq: DigestEmailFrequency = { interval: 'daily' };
const agentApprovalFreq: ImportedAgentApprovalNotificationFrequency = { mode: 'immediate' };
const createTenantUserBody: CreateTenantUserBody = {
  email: 'sara.martin@acme.co',
  name: 'Sara Martin',
  role: 'moderator',
  avatarUrl: 'https://cdn.acme.co/avatars/sara.jpg',
  notifyByEmail: true,                     // демонстриран незадължителен параметър
  digestEmailFrequency: digestFreq,
  importedAgentApprovalNotificationFrequency: agentApprovalFreq
};
const result: CreateTenantUserResponse = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---