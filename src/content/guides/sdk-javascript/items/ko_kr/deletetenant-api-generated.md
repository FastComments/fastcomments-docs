## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sure | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteTenant 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3d2c";
const id: string = "flag_8392b1a7";
const sure: string = "confirmed";

const responseWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
const responseWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
[inline-code-end]

---