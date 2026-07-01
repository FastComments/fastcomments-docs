## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 响应

返回: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV1PageLikes.h)

## 示例

[inline-code-attrs-start title = 'getV1PageLikes 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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