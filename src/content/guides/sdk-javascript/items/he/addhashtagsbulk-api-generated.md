## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | לא |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | לא |  |

## תגובה

מחזיר: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// צור מזהה tenant (פרמטר אופציונלי)
const tenantId: string = "tenant_9f8c2b7a";

// הכנה של פריטי תג בודדים
const tag1: BulkCreateHashTagsBodyTagsInner = {
  name: "product-feedback",
  label: "Product Feedback",
  color: "#1f8a70",
  description: "User suggestions and enhancement requests",
  isActive: true
};

const tag2: BulkCreateHashTagsBodyTagsInner = {
  name: "bug-report",
  label: "Bug Report",
  color: "#d64545",
  description: "User-reported defects and issues",
  isActive: true
};

// גוף הבקשה ליצירה בכמות (פרמטר אופציונלי)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// קריאה לפונקציה אסינכרונית גלובלית והקצאת התוצאה עם טיפוס
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---