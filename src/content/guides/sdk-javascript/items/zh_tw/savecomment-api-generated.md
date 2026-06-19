## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | CreateCommentParams | 是 |  |
| isLive | boolean | 否 |  |
| doSpamCheck | boolean | 否 |  |
| sendEmails | boolean | 否 |  |
| populateNotifications | boolean | 否 |  |

## 回應

回傳: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'saveComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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