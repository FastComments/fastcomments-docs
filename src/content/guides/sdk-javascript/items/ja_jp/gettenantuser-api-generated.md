## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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