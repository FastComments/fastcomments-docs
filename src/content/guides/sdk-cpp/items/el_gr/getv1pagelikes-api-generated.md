## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV1PageLikes.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getV1PageLikes'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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