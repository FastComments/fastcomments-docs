---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`GetUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f2c1b4a";
const id: string = "user_5a3b2c1d";
const result: GetUserResponse = await getUser(tenantId, id);
const status: APIStatus | undefined = result.status;
const user: User | undefined = result.user;
const digestFrequency: DigestEmailFrequency | undefined = user?.digestEmailFrequency;
const importedAgentFrequency: ImportedAgentApprovalNotificationFrequency | undefined = user?.importedAgentApprovalNotificationFrequency;
[inline-code-end]

---