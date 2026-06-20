## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 否 |  |
| createHashTagBody | CreateHashTagBody | 否 |  |

## 回應

回傳: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateHashTagResponse.h)

## 範例

[inline-code-attrs-start title = 'addHashTag 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
CreateHashTagBody createBody;
createBody.name = utility::string_t(U("release"));
createBody.createdBy = utility::string_t(U("admin@example.com"));
auto bodyOpt = boost::optional<CreateHashTagBody>(createBody);

api->addHashTag(tenantId, bodyOpt).then([](pplx::task<std::shared_ptr<CreateHashTagResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "HashTag created successfully\n";
        } else {
            auto fallback = std::make_shared<CreateHashTagResponse>();
        }
    } catch (const std::exception &e) {
        std::cerr << "AddHashTag failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---