## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Отговор

Връща: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV1PageLikes.h)

## Пример

[inline-code-attrs-start title = 'getV1PageLikes Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("article-789");
boost::optional<utility::string_t> filter;

api->getV1PageLikes(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetV1PageLikes>> task){
    try{
        auto raw = task.get();
        auto likes = std::make_shared<GetV1PageLikes>(*raw);
        auto total = likes->totalLikes;
    }catch(...){
    }
});
[inline-code-end]