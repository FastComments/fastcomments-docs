---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createCommentParams | Array<CreateCommentParams> | 예 |  |
| isLive | boolean | 아니요 |  |
| doSpamCheck | boolean | 아니요 |  |
| sendEmails | boolean | 아니요 |  |
| populateNotifications | boolean | 아니요 |  |

## 응답

반환: `Array<SaveComment200Response`

## 예제

[inline-code-attrs-start title = 'saveCommentsBulk 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---