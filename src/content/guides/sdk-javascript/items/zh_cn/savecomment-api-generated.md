## 参数

| Name | Type | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | CreateCommentParams | 是 |  |
| isLive | boolean | 否 |  |
| doSpamCheck | boolean | 否 |  |
| sendEmails | boolean | 否 |  |
| populateNotifications | boolean | 否 |  |

## 响应

返回： [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'saveComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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