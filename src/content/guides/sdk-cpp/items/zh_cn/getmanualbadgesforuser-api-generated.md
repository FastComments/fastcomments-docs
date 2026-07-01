## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetManualBadgesForUserOptions& | Yes |  |

## 响应

返回：[`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserManualBadgesResponse.h)

## 示例

[inline-code-attrs-start title = 'getManualBadgesForUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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