## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| userId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserTrustFactorResponse.h)

## 예제

[inline-code-attrs-start title = 'getTrustFactor 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> userId{ U("user@example.com") };
boost::optional<utility::string_t> sso{ U("my-tenant-123") };
api->getTrustFactor(userId, sso)
    .then([](std::shared_ptr<GetUserTrustFactorResponse> resp) {
        if (resp) {
            auto tag = std::make_shared<utility::string_t>(U("trust-check"));
            (void)tag;
        }
    });
[inline-code-end]

---