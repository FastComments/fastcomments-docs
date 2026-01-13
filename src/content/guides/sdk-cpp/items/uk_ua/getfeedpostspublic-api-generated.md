запит
tenantId
afterId

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| afterId | string | Ні |  |
| limit | int32_t | Ні |  |
| tags | vector<string | Ні |  |
| sso | string | Ні |  |
| isCrawler | bool | Ні |  |
| includeUserInfo | bool | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsPublic_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getFeedPostsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("post_abc123"));
boost::optional<int32_t> limit = boost::optional<int32_t>(20);
boost::optional<std::vector<utility::string_t>> tags = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{U("news"), U("tech")});
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<bool> isCrawler = boost::optional<bool>(false);
boost::optional<bool> includeUserInfo = boost::optional<bool>(true);
api->getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo)
.then([](pplx::task<std::shared_ptr<GetFeedPostsPublic_200_response>> t){
    try {
        std::shared_ptr<GetFeedPostsPublic_200_response> resp = t.get();
        auto marker = std::make_shared<int>(1);
        if (resp) { (void)marker; }
    } catch (const std::exception&) {
    }
});
[inline-code-end]