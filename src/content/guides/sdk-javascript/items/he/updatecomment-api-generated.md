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

[inline-code-attrs-start title = 'דוגמה ל-updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3f47b2a1";
const id: string = "comment_9a12b3c4";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Thanks for the update — I've adjusted my view accordingly."
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = false;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---