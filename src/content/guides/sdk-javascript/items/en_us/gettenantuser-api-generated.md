## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## Example

[inline-code-attrs-start title = 'getTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const id: string = "user_3a2b1c";
const response: GetTenantUserResponse = await getTenantUser(tenantId, id);
const status: APIStatus | undefined = response?.status;
const user: User | undefined = response?.user;
const digestFrequency: DigestEmailFrequency | undefined = user?.digestEmailFrequency;
const importedAgentApprovalFrequency: ImportedAgentApprovalNotificationFrequency | undefined = user?.importedAgentApprovalNotificationFrequency;
[inline-code-end]
