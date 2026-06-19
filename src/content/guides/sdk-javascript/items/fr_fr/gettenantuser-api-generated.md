## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne : [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const id: string = "user_3a2b1c";
const response: GetTenantUserResponse = await getTenantUser(tenantId, id);
const status: APIStatus | undefined = response?.status;
const user: User | undefined = response?.user;
const digestFrequency: DigestEmailFrequency | undefined = user?.digestEmailFrequency;
const importedAgentApprovalFrequency: ImportedAgentApprovalNotificationFrequency | undefined = user?.importedAgentApprovalNotificationFrequency;
[inline-code-end]

---