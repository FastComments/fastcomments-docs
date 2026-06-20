---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationUserSearchResponse.h)

## 範例

[inline-code-attrs-start title = 'getSearchUsers 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> value(boost::optional<utility::string_t>(U("john.doe@example.com")));
boost::optional<utility::string_t> sso(boost::optional<utility::string_t>(U("my-tenant-123")));
api->getSearchUsers(value, sso).then([](std::shared_ptr<ModerationUserSearchResponse> resp){
    if (!resp) return;
    auto copy = std::make_shared<ModerationUserSearchResponse>(*resp);
});
[inline-code-end]

---