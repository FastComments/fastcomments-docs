## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |

## レスポンス

戻り値: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPageByURLIdAPIResponse.h)

## 例

[inline-code-attrs-start title = 'getPageByURLId の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("home-page-789");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
api->getPageByURLId(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetPageByURLIdAPIResponse>> task){
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<GetPageByURLIdAPIResponse>();
        (void)result;
    } catch (const std::exception &ex) {
        auto err = std::make_shared<GetPageByURLIdAPIResponse>();
        (void)err;
    }
});
[inline-code-end]

---