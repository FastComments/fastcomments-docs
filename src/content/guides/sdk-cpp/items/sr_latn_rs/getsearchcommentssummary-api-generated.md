## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | const GetSearchCommentsSummaryOptions& | Da |  |

## Odgovor

Vraća: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationCommentSearchResponse.h)

## Primer

[inline-code-attrs-start title = 'getSearchCommentsSummary Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto options = std::make_shared<GetSearchCommentsSummaryOptions>();
options->query = utility::conversions::to_string_t("spam");
options->pageSize = boost::optional<int>(50);
options->pageNumber = boost::optional<int>(1);
options->fromDate = boost::none;
api->getSearchCommentsSummary(tenantId, *options).then([](pplx::task<std::shared_ptr<ModerationCommentSearchResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]