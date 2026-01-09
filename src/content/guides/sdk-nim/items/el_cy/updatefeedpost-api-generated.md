## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |
| feedPost | FeedPost | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let feedPost = FeedPost(
  title: "Local Election Results",
  content: "Updated vote counts across precincts",
  tags: @["politics", "local"],
  authorId: "journalist-32",
  isPublished: true,
  views: 124
)

let (response, httpResponse) = client.updateFeedPost(tenantId = "my-tenant-123", id = "post-456", feedPost = feedPost)

if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---