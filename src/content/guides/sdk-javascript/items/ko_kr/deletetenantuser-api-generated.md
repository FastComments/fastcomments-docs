## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| deleteComments | string | 아니오 |  |
| commentDeleteMode | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7a9f2c";
const id: string = "user_9d3f1b6a";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: APIEmptyResponse = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]