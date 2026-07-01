## 參數

| 名稱   | 類型                                 | 必填 | 說明 |
|--------|--------------------------------------|------|------|
| tenantId | string                               | 是   |      |
| options  | const GetManualBadgesForUserOptions& | 是   |      |

## 回應

返回：[`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserManualBadgesResponse.h)

## 範例

[inline-code-attrs-start title = 'getManualBadgesForUser 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetManualBadgesForUserOptions options;
options.userEmail = boost::optional<utility::string_t>(U("user@example.com"));
options.includeInactive = boost::optional<bool>(true);
api->getManualBadgesForUser(tenantId, options).then([](pplx::task<std::shared_ptr<GetUserManualBadgesResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]