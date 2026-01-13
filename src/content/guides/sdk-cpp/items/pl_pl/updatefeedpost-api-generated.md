## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| feedPost | FeedPost | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Przykład

[inline-code-attrs-start title = 'Przykład updateFeedPost'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-987654321");
FeedPost feedPost;
feedPost.title = U("Weekly status update");
feedPost.content = U("Completed the migration of the comments service to the new infra.");
feedPost.authorEmail = U("developer@acme-corp.com");
boost::optional<utility::string_t> summary = boost::optional<utility::string_t>(U("Migration complete"));
feedPost.summary = summary;
auto updateTask = api->updateFeedPost(tenantId, postId, feedPost)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto cloned = std::make_shared<FlagCommentPublic_200_response>(*resp);
            }
        } catch (...) {
        }
    });
[inline-code-end]

---