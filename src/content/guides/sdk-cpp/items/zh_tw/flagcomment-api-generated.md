## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## 範例

[inline-code-attrs-start title = 'flagComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
boost::optional<utility::string_t> userId{ utility::string_t(U("user@example.com")) };
boost::optional<utility::string_t> anonUserId;

api->flagComment(tenantId, commentId, userId, anonUserId)
.then([](std::shared_ptr<FlagComment_200_response> resp) {
    auto result = resp ? resp : std::make_shared<FlagComment_200_response>();
    (void)result;
});
[inline-code-end]