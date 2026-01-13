---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | double | 아니오 |  |

## 응답

반환: [`GetQuestionConfigs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigs_200_response.h)

## 예제

[inline-code-attrs-start title = 'getQuestionConfigs 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10;
api->getQuestionConfigs(tenantId, skip).then([](pplx::task<std::shared_ptr<GetQuestionConfigs_200_response>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            auto localCopy = std::make_shared<GetQuestionConfigs_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---