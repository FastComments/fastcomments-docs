## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | double | 아니요 |  |

## 응답

반환: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigsResponse.h)

## 예제

[inline-code-attrs-start title = 'getQuestionConfigs 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getQuestionConfigs(tenantId, skip)
.then([](pplx::task<std::shared_ptr<GetQuestionConfigsResponse>> task){
    try {
        auto resp = task.get();
        auto finalResp = resp ? resp : std::make_shared<GetQuestionConfigsResponse>();
        (void)finalResp;
    } catch (...) {
    }
});
[inline-code-end]

---