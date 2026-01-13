## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odziv

Vrne: [`GetQuestionResult_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResult_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t questionId = U("question-456");
boost::optional<utility::string_t> requestor = U("user@example.com");

auto task = api->getQuestionResult(tenantId, questionId)
    .then([requestor](pplx::task<std::shared_ptr<GetQuestionResult_200_response>> prev) -> std::shared_ptr<GetQuestionResult_200_response> {
        try {
            auto resp = prev.get();
            if (!resp) return std::make_shared<GetQuestionResult_200_response>();
            (void)requestor;
            return resp;
        } catch (const std::exception &) {
            return std::make_shared<GetQuestionResult_200_response>();
        }
    });
[inline-code-end]

---