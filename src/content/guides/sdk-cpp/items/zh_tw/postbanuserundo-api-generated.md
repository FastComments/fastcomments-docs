## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | Yes |  |
| sso | string | No |  |

## 回應

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'postBanUserUndo 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
BanUserUndoParams banParams;
banParams.userId = utility::string_t(U("user-567"));
banParams.reason = utility::string_t(U("reinstated"));
boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-abc"));

api->postBanUserUndo(tenantId, banParams, sso).then([](std::shared_ptr<APIEmptyResponse> resp){
    // 處理成功
});
[inline-code-end]