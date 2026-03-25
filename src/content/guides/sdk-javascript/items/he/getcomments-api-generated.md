## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| page | number | לא |  |
| limit | number | לא |  |
| skip | number | לא |  |
| asTree | boolean | לא |  |
| skipChildren | number | לא |  |
| limitChildren | number | לא |  |
| maxTreeDepth | number | לא |  |
| urlId | string | לא |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |
| contextUserId | string | לא |  |
| hashTag | string | לא |  |
| parentId | string | לא |  |
| direction | SortDirections | לא |  |

## תגובה

מחזיר: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // page (עמוד)
  20, // limit (הגבלה)
  0, // skip (דילוג)
  true, // asTree (כעץ)
  1, // skipChildren (דילוג על ילדים)
  3, // limitChildren (הגבלת ילדים)
  4, // maxTreeDepth (עומק מרבי של העץ)
  'articles/2026/new-product-launch', // urlId (מזהה URL)
  'user_7890', // userId (מזהה משתמש)
  'anon_4f3b2', // anonUserId (מזהה משתמש אנונימי)
  undefined, // contextUserId (מזהה משתמש בהקשר)
  '#launch', // hashTag (האשטאג)
  undefined // parentId (מזהה הורה)
);
[inline-code-end]

---