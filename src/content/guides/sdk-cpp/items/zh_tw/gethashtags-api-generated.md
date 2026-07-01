## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|------|
| tenantId | string | 是 |  |
| page | double | 否 |  |

## 回應

返回: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTagsResponse.h)

## 範例

[inline-code-attrs-start title = 'getHashTags 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> page = 2.0;

api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTagsResponse>> task) {
    try {
        auto resultPtr = task.get();
        auto response = std::make_shared<GetHashTagsResponse>(*resultPtr);
        // 使用回應
    } catch (const std::exception&) {
        // 處理錯誤
    }
});
[inline-code-end]

---