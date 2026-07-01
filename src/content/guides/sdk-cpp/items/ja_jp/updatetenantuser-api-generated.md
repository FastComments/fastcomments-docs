## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateTenantUserBody | UpdateTenantUserBody | はい |  |
| updateComments | string | いいえ |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateTenantUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
UpdateTenantUserBody body;
body.email = U("john.doe@example.com");
body.role = U("admin");
boost::optional<utility::string_t> updateComments = U("Promoted to admin");

api->updateTenantUser(tenantId, userId, body, updateComments)
    .then([](std::shared_ptr<APIEmptyResponse>){ });
[inline-code-end]