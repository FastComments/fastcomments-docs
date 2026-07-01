## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | number | לא |  |
| count | number | לא |  |
| textSearch | string | לא |  |
| byIPFromComment | string | לא |  |
| filters | string | לא |  |
| searchFilters | string | לא |  |
| sorts | string | לא |  |
| demo | boolean | לא |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## Response

מחזיר: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // עמוד
    25,                    // כמות
    "feedback",           // חיפוש טקסט
    "192.168.1.100",      // כתובת IP מההערה
    "approved",           // מסננים
    "hasReplies",         // מסנני חיפוש
    "dateDesc",           // מיון
    false,                // הדגמה
    "tenant-abc123",      // מזהה שוכר
    "sso-token-xyz"       // SSO
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]