## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 响应

返回: [`GetQuestionConfigs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigs_200_response.h)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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