---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-01";
const idOptional: string | undefined = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
const id: string = idOptional ?? "11111111-1111-1111-1111-111111111111";
const response: FlagCommentPublic200Response = await deleteQuestionConfig(tenantId, id);
[inline-code-end]

---