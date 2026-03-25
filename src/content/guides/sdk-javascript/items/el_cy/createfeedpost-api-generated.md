## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| skipDupCheck | boolean | No |  |

## Απόκριση

Επιστρέφει: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPost200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createFeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b9a";
const createFeedPostParams: CreateFeedPostParams = {
  title: "Weekly Product Update — March 2026",
  body: "We've shipped performance improvements and bug fixes across the web client. See the release notes for details.",
  authorId: "user_86fa2b",
  allowComments: true,
  media: [
    {
      url: "https://cdn.example.com/images/update-march.png",
      mimeType: "image/png",
      caption: "Performance graph",
      assets: [{ url: "https://cdn.example.com/images/update-march@2x.png", width: 1600, height: 900 }]
    }
  ],
  links: [{ url: "https://www.example.com/release-notes/march-2026", title: "Release notes" }]
};
const broadcastId: string = "broadcast_prod_updates_202603";
const isLive: boolean = false;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const result: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]