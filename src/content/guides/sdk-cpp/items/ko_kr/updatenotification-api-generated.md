## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateNotificationBody | UpdateNotificationBody | 예 |  |
| userId | string | 아니요 |  |

## 응답

반환: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'updateNotification 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notificationId = U("notif-456");
UpdateNotificationBody updateNotificationBody;
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("moderator@example.com"));
api->updateNotification(tenantId, notificationId, updateNotificationBody, userId)
.then([=](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto respCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
            std::cout << "Notification updated successfully\n";
        } else {
            std::cout << "No response received\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "Update failed: " << e.what() << "\n";
    }
});
[inline-code-end]

---