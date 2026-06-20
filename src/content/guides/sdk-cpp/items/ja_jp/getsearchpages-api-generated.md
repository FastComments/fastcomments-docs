## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| value | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## 例

[inline-code-attrs-start title = 'getSearchPages の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t value = U("getting-started");
utility::string_t sso = U("user@example.com");
boost::optional<utility::string_t> valueOpt = value;
boost::optional<utility::string_t> ssoOpt = sso;
api->getSearchPages(valueOpt, ssoOpt)
.then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> t){
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<ModerationPageSearchResponse>();
        (void)safeResp;
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]

---