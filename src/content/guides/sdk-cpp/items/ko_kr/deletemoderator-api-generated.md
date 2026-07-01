## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sendEmail | string | 아니오 |  |

## 응답

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'deleteModerator 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto moderatorId = utility::conversions::to_string_t("mod-456");
boost::optional<utility::string_t> sendEmail = utility::conversions::to_string_t("admin@example.com");
api->deleteModerator(tenantId, moderatorId, sendEmail)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // handle success
    });
[inline-code-end]