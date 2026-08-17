## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | bool | No |  |

## 回應

返回: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'putSSOUser 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData userData;
userData.email = utility::conversions::to_string_t("alice@example.com");
userData.first_name = utility::conversions::to_string_t("Alice");
userData.last_name = utility::conversions::to_string_t("Smith");
userData.role = utility::conversions::to_string_t("moderator");

api->putSSOUser(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("alice.smith"),
    userData,
    boost::optional<bool>(true)
).then([](pplx::task<std::shared_ptr<PutSSOUserAPIResponse>> t) {
    auto response = t.get();
});
[inline-code-end]