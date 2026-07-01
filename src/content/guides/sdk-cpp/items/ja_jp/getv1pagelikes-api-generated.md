## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## レスポンス

返却: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV1PageLikes.h)

## 例

[inline-code-attrs-start title = 'getV1PageLikes の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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