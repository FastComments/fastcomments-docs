## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Yes |  |
| updateComments | string | No |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'replaceTenantUser 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto userId = utility::string_t(U("john.doe@example.com"));
ReplaceTenantUserBody replaceBody;
replaceBody.email = utility::string_t(U("john.doe@example.com"));
replaceBody.role = utility::string_t(U("admin"));
boost::optional<utility::string_t> updateComments = utility::string_t(U("Promoted to admin"));
api->replaceTenantUser(tenantId, userId, replaceBody, updateComments)
    .then([](std::shared_ptr<APIEmptyResponse> resp){ });
[inline-code-end]