## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateAPISSOUserData | UpdateAPISSOUserData | はい |  |
| updateComments | bool | いいえ |  |

## レスポンス

戻り値: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## 例

[inline-code-attrs-start title = 'putSSOUser の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
UpdateAPISSOUserData updateData;
updateData.displayName = U("Jane Doe");
updateData.email = U("user@example.com");
boost::optional<bool> updateComments = true;
api->putSSOUser(tenantId, id, updateData, updateComments)
.then([](std::shared_ptr<PutSSOUserAPIResponse> resp){
    if(!resp){ std::cout << "putSSOUser returned null\n"; return; }
    auto copy = std::make_shared<PutSSOUserAPIResponse>(*resp);
    std::cout << "SSO user updated successfully\n";
});
[inline-code-end]

---