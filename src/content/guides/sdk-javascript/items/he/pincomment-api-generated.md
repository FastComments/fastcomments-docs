## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| broadcastId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-pinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-72";
const commentId: string = "cmt_8f3a2b4c9d";
const broadcastId: string = "live_2026-06-15_21z";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload.signature";

const responseNoSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]

---