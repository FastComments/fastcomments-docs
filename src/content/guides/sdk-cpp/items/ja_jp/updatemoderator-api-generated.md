## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateModeratorBody | UpdateModeratorBody | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateModerator の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto updateBody = std::make_shared<UpdateModeratorBody>();
updateBody->email = utility::string_t(U("moderator@example.com"));
updateBody->displayName = boost::optional<utility::string_t>(utility::string_t(U("Jane Moderator")));
updateBody->role = boost::optional<utility::string_t>(utility::string_t(U("senior-moderator")));
updateBody->active = boost::optional<bool>(true);
api->updateModerator(utility::string_t(U("my-tenant-123")), utility::string_t(U("moderator-456")), *updateBody)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        if (resp) {
            auto result = resp;
        }
        return resp;
    });
[inline-code-end]

---