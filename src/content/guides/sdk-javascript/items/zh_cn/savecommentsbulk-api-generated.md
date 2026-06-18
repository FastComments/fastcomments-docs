## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createCommentParams | Array<CreateCommentParams> | 是 |  |
| isLive | boolean | 否 |  |
| doSpamCheck | boolean | 否 |  |
| sendEmails | boolean | 否 |  |
| populateNotifications | boolean | 否 |  |

## 响应

返回: `Array<SaveComment200Response`

## 示例

[inline-code-attrs-start title = 'saveCommentsBulk 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42a1b7";
const mentions: CommentUserMentionInfo[] = [{ userId: "user_2b9", displayName: "Alex Chen" }];
const hashtags: CommentUserHashTagInfo[] = [{ tag: "performance" }];
const createCommentParams: CreateCommentParams[] = [
  {
    content: "Thanks for the detailed article — the alternative approach worked for me.",
    authorId: "user_8f3c2",
    authorName: "Maya Patel",
    authorEmail: "maya.patel@example.com",
    url: "/articles/optimizing-ts-performance",
    createdAt: new Date().toISOString(),
    mentions,
    hashtags
  }
];
const isLive: boolean = true;
const doSpamCheck: boolean = false;
const sendEmails: boolean = true;
const populateNotifications: boolean = true;
const result: Array<SaveComment200Response> = await saveCommentsBulk(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]