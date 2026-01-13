## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | double | לא |  |

## תגובה

מחזיר: [`GetQuestionConfigs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigs_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getQuestionConfigs'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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