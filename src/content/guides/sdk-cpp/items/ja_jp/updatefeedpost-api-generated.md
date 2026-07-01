## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| feedPost | FeedPost | はい |  |

## 応答

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateFeedPost の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U"my-tenant-123";
utility::string_t postId = U"post-456";

FeedPost feedPost;
feedPost.title = U"Breaking News";
feedPost.content = U"Details of the update go here.";
feedPost.imageUrl = boost::optional<utility::string_t>(U"https://example.com/image.jpg");

api->updateFeedPost(tenantId, postId, feedPost)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception& ex) {
        }
    });
[inline-code-end]

---