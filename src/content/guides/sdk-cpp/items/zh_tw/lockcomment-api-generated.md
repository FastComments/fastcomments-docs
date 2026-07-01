## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'lockComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-987");
auto broadcastId = utility::conversions::to_string_t("broadcast-654");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-xyz");

api->lockComment(tenantId, commentId, broadcastId, sso).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]