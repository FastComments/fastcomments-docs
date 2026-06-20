---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## 예제

[inline-code-attrs-start title = 'getSearchPages 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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