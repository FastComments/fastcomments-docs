## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| limit | double | 否 |  |
| skip | double | 否 |  |

## 回應

回傳: [`GetUserBadgeProgressList_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressList_200_response.h)

## 範例

[inline-code-attrs-start title = 'getUserBadgeProgressList 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<double> limit = boost::optional<double>(50);
boost::optional<double> skip = boost::optional<double>(0);

api->getUserBadgeProgressList(tenantId, userId, limit, skip)
.then([](pplx::task<std::shared_ptr<GetUserBadgeProgressList_200_response>> t){
    try {
        auto resp = t.get();
        auto copied = std::make_shared<GetUserBadgeProgressList_200_response>(*resp);
        std::cout << "Badge progress entries received\n";
    } catch (const std::exception &e) {
        std::cerr << "Failed to get badge progress: " << e.what() << '\n';
    }
});
[inline-code-end]

---