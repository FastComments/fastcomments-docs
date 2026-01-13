## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentText_200_response.h)

## 例

[inline-code-attrs-start title = 'getCommentText の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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