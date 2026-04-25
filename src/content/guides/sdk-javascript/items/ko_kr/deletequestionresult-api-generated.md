---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteQuestionResult 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantIdEnv: string | undefined = process.env.FASTCOMMENTS_TENANT_ID;
const tenantId: string = tenantIdEnv ?? 'tenant_78b3f2';
const id: string = 'qres-9f2a3b1c';
const response: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, id);
[inline-code-end]

---