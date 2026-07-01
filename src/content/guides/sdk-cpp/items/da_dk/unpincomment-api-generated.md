## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Svar

Returnerer: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Eksempel

[inline-code-attrs-start title = 'unPinComment Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto task = api->unPinComment(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("cmt-456789"),
    utility::conversions::to_string_t("broadcast-001"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"))
);
task.then([](std::shared_ptr<ChangeCommentPinStatusResponse> resp){
    auto result = std::make_shared<ChangeCommentPinStatusResponse>(*resp);
});
[inline-code-end]