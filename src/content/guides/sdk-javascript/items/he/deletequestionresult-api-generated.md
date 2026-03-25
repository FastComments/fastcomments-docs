## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org-82479";
const questionId: string = "q-6a3d2e1f";
const includeArchived?: boolean = false; // פרמטר אופציונלי המדגים בחירת יעד חלופי
const targetId: string = includeArchived ? "q-archived-112233" : questionId;
const result: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, targetId);
[inline-code-end]

---