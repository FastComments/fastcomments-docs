---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| r_namespace | string | 是 |  |
| component | string | 是 |  |
| locale | string | 否 |  |
| useFullTranslationIds | bool | 否 |  |

## 回應

回傳：[`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTranslationsResponse.h)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> locale(U("en-US"));
boost::optional<bool> useFullTranslationIds(true);
api->getTranslations(U("my-tenant-123"), U("comment-widget"), locale, useFullTranslationIds)
.then([](pplx::task<std::shared_ptr<GetTranslationsResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copied = std::make_shared<GetTranslationsResponse>(*resp);
        }
    } catch (const std::exception&) {
        throw;
    }
});
[inline-code-end]

---