---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | Array<string> | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserReactsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-4f2b9c';
const postIds: Array<string> = ['post_7a1f9e', 'post_2b3c88'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTYwOTQ0MDAwMH0.signature';

const response: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
[inline-code-end]

---