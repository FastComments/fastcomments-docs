## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| dir | int32_t | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentVoteUserNames Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto task = api->getCommentVoteUserNames(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("comment-456"),
    static_cast<int32_t>(1),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("sso-token"))
).then([](pplx::task<std::shared_ptr<GetCommentVoteUserNamesSuccessResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){ }
});
[inline-code-end]