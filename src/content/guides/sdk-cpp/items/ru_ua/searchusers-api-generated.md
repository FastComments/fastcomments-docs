## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| options | const SearchUsersOptions& | Да |  |

## Ответ

Возвращает: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsersResult.h)

## Пример

[inline-code-attrs-start title = 'searchUsers Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
SearchUsersOptions options;
options.query = U("john.doe@example.com");
options.page = boost::optional<int>(1);
options.pageSize = boost::optional<int>(20);
api->searchUsers(tenantId, urlId, options).then([](pplx::task<std::shared_ptr<SearchUsersResult>> task){
    try{
        auto result = task.get();
    }catch(const std::exception&){}
});
[inline-code-end]