## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments_corp';
const id: string = 'user_9f8b7c6d-5e4a-3b2c-1f0e-123456789abc';
const response: GetUser200Response = await getUser(tenantId, id);
const userEmail: string | undefined = response.user?.email;
const displayName: string | undefined = response.user?.displayName
[inline-code-end]