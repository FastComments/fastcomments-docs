---
## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## תגובה

מחזיר: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchConfigs() {
  const tenantId: string = "tenant_12345";

  const firstPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId);
  const secondPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, 20);
}
[inline-code-end]

---