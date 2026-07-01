## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetApiIdsOptions& | Yes |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## Приклад

[inline-code-attrs-start title = 'getApiIds Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetApiIdsOptions options;
options.limit = boost::optional<int>(100);
options.cursor = boost::optional<utility::string_t>(U("next-page-token"));
api->getApiIds(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentIdsResponse>> t){
    try{
        auto response = t.get();
        auto ids = response->commentIds;
    }catch(const std::exception&){
    }
});
[inline-code-end]