## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Réponse

Renvoie : [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV1PageLikes.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getV1PageLikes'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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