## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| errorId | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optTemplateId = utility::conversions::to_string_t("template-456");
api->deleteEmailTemplateRenderError(
    utility::conversions::to_string_t("my-tenant-123"),
    *optTemplateId,
    utility::conversions::to_string_t("error-789"))
    .then([](std::shared_ptr<APIEmptyResponse>) {})
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (...) {}
    });
[inline-code-end]