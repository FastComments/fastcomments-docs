## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserResponse.ts)

## 예제

[inline-code-attrs-start title = 'getUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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