---  
## 参数  

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 响应  

返回：[`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersCountResponse.h)

## 示例  

[inline-code-attrs-start title = 'getCounts 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
api->getCounts(
    utility::conversions::to_string_t("my-tenant-123"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("john.doe@example.com"))
).then([](pplx::task<std::shared_ptr<GetBannedUsersCountResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]  

---