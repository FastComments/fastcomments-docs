## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| includeByUserIdAndEmail | bool | Не |  |
| includeByIP | bool | Не |  |
| includeByEmailDomain | bool | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`PreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PreBanSummary.h)

## Пример

[inline-code-attrs-start title = 'getPreBanSummary Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = utility::conversions::to_string_t("cmt-987654321");
boost::optional<bool> includeByUserIdAndEmail = true;
boost::optional<bool> includeByIP = true;
boost::optional<bool> includeByEmailDomain = false;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("my-tenant-123-sso-token");
api->getPreBanSummary(commentId, includeByUserIdAndEmail, includeByIP, includeByEmailDomain, sso)
    .then([](pplx::task<std::shared_ptr<PreBanSummary>> t) {
        try {
            auto summary = t.get();
            if (!summary) summary = std::make_shared<PreBanSummary>();
            std::cout << "PreBanSummary retrieved" << std::endl;
        } catch (const std::exception &e) {
            std::cerr << "Error fetching PreBanSummary: " << e.what() << std::endl;
        }
    });
[inline-code-end]

---