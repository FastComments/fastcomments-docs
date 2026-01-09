## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| limit | double | No |  |
| skip | double | No |  |

## Yanıt

Dönüş değeri: [`GetUserBadgeProgressList_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressList_200_response.h)

## Örnek

[inline-code-attrs-start title = 'getUserBadgeProgressList Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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