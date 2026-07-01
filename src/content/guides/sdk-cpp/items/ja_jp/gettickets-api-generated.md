## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | const GetTicketsOptions& | はい |  |

## 応答

返却: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicketsResponse.h)

## 例

[inline-code-attrs-start title = 'getTickets の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetTicketsOptions{};
options.page = boost::optional<int>(1);
options.status = boost::optional<utility::string_t>(U("open"));
api->getTickets(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<GetTicketsResponse>> task) {
    try {
        auto response = task.get();
        auto responseCopy = std::make_shared<GetTicketsResponse>(*response);
    } catch (const std::exception&) {
    }
});
[inline-code-end]