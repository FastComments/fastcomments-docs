## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| commentsByIdsParams | CommentsByIdsParams | Yes |  |
| sso | string | No |  |

## Réponse

Renvoie : [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple postCommentsByIds'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
CommentsByIdsParams params;
params.commentIds = {U("cmt-001"), U("cmt-002")};
boost::optional<utility::string_t> sso = U("sso-token-abc");
api->postCommentsByIds(tenantId, params, sso).then([](pplx::task<std::shared_ptr<ModerationAPIChildCommentsResponse>> t){
    try{
        auto resp = t.get();
        auto copy = std::make_shared<ModerationAPIChildCommentsResponse>(*resp);
    }catch(const std::exception&){ }
});
[inline-code-end]