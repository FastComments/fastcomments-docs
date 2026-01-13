## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createAPISSOUserData | CreateAPISSOUserData | 是 |  |

## 响应

返回：[`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddSSOUserAPIResponse.h)

## 示例

[inline-code-attrs-start title = 'addSSOUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto createData = std::make_shared<CreateAPISSOUserData>();
createData->email = utility::string_t(U("alice@example.com"));
createData->externalUserId = utility::string_t(U("okta|987654321"));
createData->displayName = boost::optional<utility::string_t>(U("Alice Johnson"));
createData->roles = std::vector<utility::string_t>{ U("moderator"), U("editor") };
api->addSSOUser(tenantId, createData)
.then([](pplx::task<std::shared_ptr<AddSSOUserAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            (void)resp;
        }
    } catch (...) {
    }
});
[inline-code-end]

---