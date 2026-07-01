## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const GetPreBanSummaryOptions& | Yes |  |

## Ответ

Возвращает: [`PreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PreBanSummary.h)

## Пример

[inline-code-attrs-start title = 'Пример getPreBanSummary'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
GetPreBanSummaryOptions options;
options.locale = boost::optional<utility::string_t>{utility::conversions::to_string_t("en-US")};

api->getPreBanSummary(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PreBanSummary>> t) {
        try {
            auto summary = t.get();
            // обработать summary
        } catch (const std::exception&) {
            // обработать ошибку
        }
    });
[inline-code-end]