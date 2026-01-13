## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentText_200_response.h)

## 예제

[inline-code-attrs-start title = 'getCommentText 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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