## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_0a1b2c3d';
const moderators: GetModerators200Response = await getModerators(tenantId);
const skip: number = 20;
const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, skip);
[inline-code-end]

---