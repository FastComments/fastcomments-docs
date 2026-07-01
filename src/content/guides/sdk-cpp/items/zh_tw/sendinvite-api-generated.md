## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| fromName | string | 是 |  |

## 回應

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'sendInvite 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> cc = utility::conversions::to_string_t("cc@example.com");
api->sendInvite(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("invitee@example.com"),
    utility::conversions::to_string_t("John Doe")
).then([](std::shared_ptr<APIEmptyResponse> resp) {
    // 處理成功的邀請
});
[inline-code-end]