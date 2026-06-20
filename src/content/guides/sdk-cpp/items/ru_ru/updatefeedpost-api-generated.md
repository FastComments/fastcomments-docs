---
## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| feedPost | FeedPost | Да |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример updateFeedPost'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto postId = utility::string_t(U("feedpost-456"));
auto post = std::make_shared<FeedPost>();
post->title = utility::string_t(U("Weekly Update"));
post->content = utility::string_t(U("This week's changes include bug fixes and performance improvements."));
post->authorEmail = boost::optional<utility::string_t>(utility::string_t(U("author@example.com")));
post->published = boost::optional<bool>(true);
api->updateFeedPost(tenantId, postId, *post)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
    try {
        auto resp = t.get();
        (void)resp;
    } catch (...) {}
});
[inline-code-end]

---