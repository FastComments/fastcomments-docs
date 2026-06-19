## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| postId | string | כן |  |
| updateFeedPostParams | UpdateFeedPostParams | כן |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b2f4a";
const postId: string = "post_4f8c21";
const updateFeedPostParams: UpdateFeedPostParams = {
  title: "Weekly product update",
  content: "We've shipped improvements to search relevance and mobile layout.",
  media: [
    {
      type: "image",
      assets: [{ url: "https://cdn.company.com/images/update-cover.jpg", width: 1200, height: 627 }]
    }
  ],
  links: [{ url: "https://company.com/blog/release-notes", title: "Release notes" }],
  published: true
};
const broadcastId: string = "broadcast_2026_06_19";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature";
const response: CreateFeedPostResponse = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]

---