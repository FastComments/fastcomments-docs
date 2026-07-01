## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## 응답

반환: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptySuccessResponse.h)

## 예제

[inline-code-attrs-start title = 'updateUserBadge 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
UpdateUserBadgeParams params;
params.badgeId = utility::conversions::to_string_t("vip-badge");
params.expiration = boost::optional<utility::datetime>(utility::datetime::from_string(U("2024-12-31T23:59:59Z")));
api->updateUserBadge(tenantId, userId, params)
    .then([](std::shared_ptr<APIEmptySuccessResponse> resp){
        std::cout << "Badge updated successfully\n";
    })
    .catch([](std::exception const& e){
        std::cerr << "Error updating badge: " << e.what() << "\n";
    });
[inline-code-end]

---