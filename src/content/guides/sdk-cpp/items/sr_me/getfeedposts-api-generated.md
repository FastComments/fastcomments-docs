req
tenantId
afterId

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | int32_t | Не |  |
| tags | vector<string | Не |  |

## Одговор

Враћа: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsResponse.h)

## Пример

[inline-code-attrs-start title = 'getFeedPosts Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("post_987"));
boost::optional<int32_t> limit = boost::optional<int32_t>(50);
boost::optional<std::vector<utility::string_t>> tags = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{ U("release"), U("security") });
api->getFeedPosts(tenantId, afterId, limit, tags)
    .then([](pplx::task<std::shared_ptr<GetFeedPostsResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<GetFeedPostsResponse>();
            std::cout << "Received feed response" << std::endl;
        } catch (const std::exception& ex) {
            std::cerr << "getFeedPosts failed: " << ex.what() << std::endl;
        }
    });
[inline-code-end]