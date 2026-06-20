## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| feedPost | FeedPost | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let feedPost: FeedPost = FeedPost(title: "City Council Approves New Waterfront Park",
  content: "The council voted 5-2 to approve funding and a timeline for construction.",
  tags: @["local", "parks", "city"],
  published: true)

let (response, httpResponse) = client.updateFeedPost(tenantId = "my-tenant-123", id = "post-456", feedPost = feedPost)

if response.isSome:
  let apiResp = response.get()
  echo "Feed post updated successfully"
  echo apiResp
else:
  echo "Failed to update feed post"
  echo httpResponse
[inline-code-end]

---