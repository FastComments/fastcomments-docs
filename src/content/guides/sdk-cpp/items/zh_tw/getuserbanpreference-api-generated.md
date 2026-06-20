## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 回應

回傳: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIModerateGetUserBanPreferencesResponse.h)

## 範例

[inline-code-attrs-start title = 'getUserBanPreference 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("my-tenant-123")));
api->getUserBanPreference(sso).then([](std::shared_ptr<APIModerateGetUserBanPreferencesResponse> resp){
    auto prefs = resp ? resp : std::make_shared<APIModerateGetUserBanPreferencesResponse>();
    (void)prefs;
});
[inline-code-end]

---