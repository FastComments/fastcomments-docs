## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createCommentParams | CreateCommentParams | はい |  |
| isLive | boolean | いいえ |  |
| doSpamCheck | boolean | いいえ |  |
| sendEmails | boolean | いいえ |  |
| populateNotifications | boolean | いいえ |  |

## レスポンス

返却値: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'saveCommentの例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_98765";
const createCommentParams: CreateCommentParams = {
  content: "This fixed my build pipeline — switching the environment variable resolved the missing dependency.",
  threadId: "blog-post-2026-06-19-ci-troubleshooting",
  userSession: { userId: "user_7542", displayName: "Aisha Khan", email: "aisha.k@example.com" }
};
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const response: APISaveCommentResponse = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]

---