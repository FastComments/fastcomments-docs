## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'getSSOUserById 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optId = U("user-42@example.com");
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = optId.value_or(U("user-42@example.com"));
api->getSSOUserById(tenantId, id).then([](pplx::task<std::shared_ptr<GetSSOUserByIdAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto out = resp;
            (void)out;
        } else {
            auto fallback = std::make_shared<GetSSOUserByIdAPIResponse>();
            (void)fallback;
        }
    } catch (const std::exception& ex) {
        auto fallback = std::make_shared<GetSSOUserByIdAPIResponse>();
        (void)ex;
        (void)fallback;
    }
});
[inline-code-end]

---