## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updatableCommentParams | UpdatableCommentParams | כן |  |
| contextUserId | string | לא |  |
| doSpamCheck | boolean | לא |  |
| isLive | boolean | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3c1b2a';
const commentId: string = 'cmt_8d9f2a4b';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Updating this comment to clarify the feature behavior and include a timestamp.',
  metadata: { category: 'support', editedReason: 'clarify instructions' },
  visible: true
};
const contextUserId: string = 'user_42';
const doSpamCheck: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, commentId, updatableCommentParams, contextUserId, doSpamCheck);
[inline-code-end]

---