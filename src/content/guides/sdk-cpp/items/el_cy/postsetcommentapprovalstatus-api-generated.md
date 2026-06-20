## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| commentId | string | Ναι |  |
| approved | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentApprovedResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα χρήσης postSetCommentApprovalStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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