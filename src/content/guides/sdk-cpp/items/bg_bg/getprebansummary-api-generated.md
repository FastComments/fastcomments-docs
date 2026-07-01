## Parameters

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| options | const GetPreBanSummaryOptions& | Да |  |

## Отговор

Връща: [`PreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PreBanSummary.h)

## Пример

[inline-code-attrs-start title = 'Пример за getPreBanSummary'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
GetPreBanSummaryOptions options;
options.locale = boost::optional<utility::string_t>{utility::conversions::to_string_t("en-US")};

api->getPreBanSummary(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PreBanSummary>> t) {
        try {
            auto summary = t.get();
            // обработи резюме
        } catch (const std::exception&) {
            // обработи грешка
        }
    });
[inline-code-end]