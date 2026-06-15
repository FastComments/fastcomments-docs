## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sure | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteTenant 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_742b9c';
const flagId: string = 'flag_1a2b3c';
const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId);
const sureConfirmation: string = 'confirmed';
const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId, sureConfirmation);
[inline-code-end]

---