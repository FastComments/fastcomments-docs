---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | double | いいえ |  |

## レスポンス

返却値: [`GetHashTags_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTags_200_response.h)

## 例

[inline-code-attrs-start title = 'getHashTags の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> page = 2.0;
auto defaultResp = std::make_shared<GetHashTags_200_response>();
api->getHashTags(tenantId, page).then([defaultResp](pplx::task<std::shared_ptr<GetHashTags_200_response>> t) {
    try {
        auto resp = t.get();
        if(!resp) resp = defaultResp;
        std::cout << "getHashTags completed; response object " << (resp ? "present" : "absent") << std::endl;
    } catch(const std::exception& e) {
        std::cout << "getHashTags error: " << e.what() << std::endl;
    }
});
[inline-code-end]

---