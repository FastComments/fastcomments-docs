---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |

## 응답

반환: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPagesAPIResponse.h)

## 예시

[inline-code-attrs-start title = 'getPages 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
api->getPages(tenantId)
    .then([](pplx::task<std::shared_ptr<GetPagesAPIResponse>> task) {
        try {
            auto response = task.get();
            boost::optional<utility::string_t> nextCursor = response->nextCursor;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---