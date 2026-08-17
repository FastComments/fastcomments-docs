req
tenantId
afterId

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| options | const GetFeedPostsPublicOptions& | Yes |  |

## Одговор

Враћа: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer getFeedPostsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetFeedPostsPublicOptions{};
options.limit = boost::optional<int>{20};
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){
    try{
        auto response = task.get();
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);
        // Користите обрађени резултат по потреби
    }catch(const std::exception&){
        // Обради грешку
    }
});
[inline-code-end]