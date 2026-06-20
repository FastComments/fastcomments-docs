---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| sso | string | いいえ |  |

## レスポンス

戻り値: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIModerateGetUserBanPreferencesResponse.h)

## 例

[inline-code-attrs-start title = 'getUserBanPreference の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("my-tenant-123")));
api->getUserBanPreference(sso).then([](std::shared_ptr<APIModerateGetUserBanPreferencesResponse> resp){
    auto prefs = resp ? resp : std::make_shared<APIModerateGetUserBanPreferencesResponse>();
    (void)prefs;
});
[inline-code-end]

---