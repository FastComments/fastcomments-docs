## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니요 |  |
| urlId | string | 아니요 |  |
| fromCommentId | string | 아니요 |  |
| viewed | bool | 아니요 |  |
| type | string | 아니요 |  |
| skip | double | 아니요 |  |

## 응답

반환: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## 예제

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> urlId(U("/articles/2026/new-feature"));
boost::optional<utility::string_t> fromCommentId(U("cmt-98765"));
boost::optional<bool> viewed(true);
boost::optional<utility::string_t> type(U("reply"));
boost::optional<double> skip(0.0);

api->getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip)
.then([](std::shared_ptr<GetNotificationsResponse> resp){
    auto holder = std::make_shared<GetNotificationsResponse>();
    holder = resp;
    if (holder) std::cout << "Received notifications\n";
    return holder;
});
[inline-code-end]

---