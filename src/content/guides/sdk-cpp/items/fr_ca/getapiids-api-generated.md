## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filters | string | Non |  |
| searchFilters | string | Non |  |
| afterId | string | Non |  |
| demo | bool | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getApiIds'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getApiIds(
    boost::optional<utility::string_t>(U("spam report")),
    boost::optional<utility::string_t>(U("203.0.113.45")),
    boost::optional<utility::string_t>(U("status:pending")),
    boost::optional<utility::string_t>(U("tenant:my-tenant-123")),
    boost::optional<utility::string_t>(U("cmt_987654321")),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentIdsResponse>> task){
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<ModerationAPIGetCommentIdsResponse>();
        (void)safeResp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]

---