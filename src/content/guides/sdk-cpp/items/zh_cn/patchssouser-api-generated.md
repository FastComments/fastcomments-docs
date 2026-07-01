## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateAPISSOUserData | UpdateAPISSOUserData | 是 |  |
| updateComments | bool | 否 |  |

## 响应

返回：[`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## 示例

[inline-code-attrs-start title = 'patchSSOUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
UpdateAPISSOUserData updateData;
updateData.email = U("jane.doe@example.com");
updateData.displayName = U("Jane Doe");
boost::optional<bool> updateComments = true;

api->patchSSOUser(tenantId, userId, updateData, updateComments)
    .then([](pplx::task<std::shared_ptr<PatchSSOUserAPIResponse>> t){
        try{
            auto resp = t.get();
            (void)resp;
        }catch(const std::exception&){
        }
    });
[inline-code-end]