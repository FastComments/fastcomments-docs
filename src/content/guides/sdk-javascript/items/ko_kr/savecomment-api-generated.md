## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createCommentParams | CreateCommentParams | 예 |  |
| isLive | boolean | 아니오 |  |
| doSpamCheck | boolean | 아니오 |  |
| sendEmails | boolean | 아니오 |  |
| populateNotifications | boolean | 아니오 |  |

## 응답

반환: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'saveComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-42";
const createCommentParams: CreateCommentParams = {
  threadId: "article-2026-06-0142",
  content: "Great write-up — I followed the migration steps and everything worked as described.",
  userId: "u_9c72b",
  userName: "Ava R.",
  userAvatarUrl: "https://cdn.example.com/avatars/u_9c72b.png",
  metadata: { platform: "web", locale: "en-US" }
};
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const result: SaveComment200Response = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]

---