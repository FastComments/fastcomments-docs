## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 回應

返回：[`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigsResponse.h)

## 範例

[inline-code-attrs-start title = 'getQuestionConfigs 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getQuestionConfigs(tenantId, skip).then([](std::shared_ptr<GetQuestionConfigsResponse> resp){
    auto config = std::make_shared<GetQuestionConfigsResponse>(*resp);
});
[inline-code-end]

---