## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| r_namespace | string | 예 |  |
| component | string | 예 |  |
| options | const GetTranslationsOptions& | 예 |  |

## 응답

반환: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTranslationsResponse.h)

## 예제

[inline-code-attrs-start title = 'getTranslations 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t ns = U("my-tenant-123");
utility::string_t comp = U("comments");
auto optsPtr = std::make_shared<GetTranslationsOptions>();
optsPtr->language = boost::make_optional(U("en"));
optsPtr->fallback = boost::none;
api->getTranslations(ns, comp, *optsPtr)
    .then([](pplx::task<std::shared_ptr<GetTranslationsResponse>> t) {
        try {
            auto resp = t.get();
        } catch (const std::exception& e) {
        }
    });
[inline-code-end]