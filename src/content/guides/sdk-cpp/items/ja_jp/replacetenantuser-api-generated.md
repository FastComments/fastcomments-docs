## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| replaceTenantUserBody | ReplaceTenantUserBody | はい |  |
| updateComments | string | いいえ |  |

## レスポンス

返却: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'replaceTenantUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
ReplaceTenantUserBody replaceBody;
boost::optional<utility::string_t> updateComments = boost::optional<utility::string_t>(U("true"));
api->replaceTenantUser(tenantId, id, replaceBody, updateComments)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<FlagCommentPublic_200_response>(*resp);
    }
});
[inline-code-end]

---