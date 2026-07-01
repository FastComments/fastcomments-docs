запрос
tenantId
afterId

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetFeedPostsPublicOptions& | Да |  |

## Ответ

Возвращает: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getFeedPostsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetFeedPostsPublicOptions{};
options.limit = boost::optional<int>{20};
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){
    try{
        auto response = task.get();
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);
        // Используйте processed по необходимости
    }catch(const std::exception&){
        // Обработать ошибку
    }
});
[inline-code-end]