## 參數

| 名稱 | Type | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`GetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentText_200_response.h)

## 範例

[inline-code-attrs-start title = 'getCommentText 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t commentId(U("cmt-456"));
boost::optional<utility::string_t> editKey(utility::string_t(U("edit-key-789")));
boost::optional<utility::string_t> sso(utility::string_t(U("user@example.com")));
auto task = api->getCommentText(tenantId, commentId, editKey, sso)
    .then([](std::shared_ptr<GetCommentText_200_response> resp){
        if (resp) {
            std::cout << "Comment text retrieved\n";
        } else {
            std::cout << "Comment not found\n";
        }
    });
task.wait();
[inline-code-end]

---