## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetApiCommentsOptions& | Да |  |

## Ответ

Возвращает: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetApiCommentsOptions{};
options.page = boost::make_optional(2);
options.authorEmail = boost::make_optional<utility::string_t>(U("user@example.com"));
options.includeDeleted = boost::make_optional(false);

api->getApiComments(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentsResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]