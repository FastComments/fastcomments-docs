## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 响应

返回: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigsResponse.h)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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