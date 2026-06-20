---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| approved | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentApprovedResponse.h)

## 예제

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-12345");
boost::optional<bool> approved = boost::optional<bool>(true);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto task = api->postSetCommentApprovalStatus(commentId, approved, sso)
.then([](std::shared_ptr<SetCommentApprovedResponse> resp){
    auto result = resp ? resp : std::make_shared<SetCommentApprovedResponse>();
    return result;
});
[inline-code-end]

---