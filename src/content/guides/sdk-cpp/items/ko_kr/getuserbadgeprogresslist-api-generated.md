## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| options | const GetUserBadgeProgressListOptions& | 예 |  |

## 응답

반환: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIGetUserBadgeProgressListResponse.h)

## 예시

[inline-code-attrs-start title = 'getUserBadgeProgressList 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetUserBadgeProgressListOptions options;
options.userId = U("user@example.com");
options.page = boost::optional<int>(1);
options.pageSize = boost::optional<int>(20);
api->getUserBadgeProgressList(tenantId, options)
    .then([](std::shared_ptr<APIGetUserBadgeProgressListResponse> resp) {
        if (!resp) return;
        for (const auto& badge : resp->badges) {
            // 배지 처리
        }
    });
[inline-code-end]

---