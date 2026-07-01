## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| broadcastId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Örnek

[inline-code-attrs-start title = 'unPinComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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