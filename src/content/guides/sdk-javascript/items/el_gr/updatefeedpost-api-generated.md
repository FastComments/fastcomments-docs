## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| feedPost | FeedPost | Yes |  |

## Απάντηση

Επιστρέφει: [`UpdateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateFeedPostResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'updateFeedPost Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3421";
const postId: string = "feedpost_a9b8c7";

const feedPost: FeedPost = {
  content: "We've refreshed the announcement with the latest project milestones.",
  media: [
    {
      type: "image",
      url: "https://assets.example.com/images/milestone.png",
      caption: "Project Milestones"
    } as FeedPostMediaItem
  ],
  link: {
    url: "https://example.com/project-updates",
    title: "Project Updates",
    description: "Read about the recent progress and upcoming goals."
  } as FeedPostLink
};

const result: UpdateFeedPostResponse = await updateFeedPost(tenantId, postId, feedPost);
[inline-code-end]