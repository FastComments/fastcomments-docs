## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateQuestionResultBody | UpdateQuestionResultBody | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f2b9';
const id: string = 'question_3a7c1d';
const updateQuestionResultBody: UpdateQuestionResultBody = {
  result: { verdict: 'helpful', confidence: 0.92 },
  reviewer: { id: 'mod_102', name: 'Aisha Rahman' },
  notifyUser: true // פרמטר אופציונלי כלול
};
const response: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---