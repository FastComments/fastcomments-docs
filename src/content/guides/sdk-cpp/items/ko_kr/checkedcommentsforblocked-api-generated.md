## 매개변수

| 이름 | Type | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentIds | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`CheckedCommentsForBlocked_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckedCommentsForBlocked_200_response.h)

## 예제

[inline-code-attrs-start title = 'checkedCommentsForBlocked 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentIds = utility::conversions::to_string_t("cmt-456,cmt-789");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");
auto task = api->checkedCommentsForBlocked(tenantId, commentIds, sso)
    .then([=](std::shared_ptr<CheckedCommentsForBlocked_200_response> resp) -> std::shared_ptr<CheckedCommentsForBlocked_200_response> {
        if (!resp) return std::make_shared<CheckedCommentsForBlocked_200_response>();
        return resp;
    });
[inline-code-end]

---