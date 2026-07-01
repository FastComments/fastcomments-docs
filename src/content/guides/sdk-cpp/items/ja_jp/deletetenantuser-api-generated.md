## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| options | const DeleteTenantUserOptions& | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'deleteTenantUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DeleteTenantUserOptions options;
options.reason = boost::optional<utility::string_t>(U("User requested deletion"));

api->deleteTenantUser(U("my-tenant-123"), U("user@example.com"), options)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        (void)resp;
    });
[inline-code-end]