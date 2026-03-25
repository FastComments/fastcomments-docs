## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| errorId | string | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b4c2a';
const templateEnvironment: string | undefined = 'production'; // 선택적 환경 선택기
const id: string = `emailTemplates/${templateEnvironment ?? 'staging'}/welcome_v2`;
const errorId: string = 'err_5a9d2f1c';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
console.log(result);
[inline-code-end]

---