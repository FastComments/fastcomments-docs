## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| textSearch | string | Не |  |
| byIPFromComment | string | Не |  |
| filters | string | Не |  |
| searchFilters | string | Не |  |
| afterId | string | Не |  |
| demo | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## Пример

[inline-code-attrs-start title = 'getApiIds Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getApiIds(
    boost::optional<utility::string_t>(U("spam report")),
    boost::optional<utility::string_t>(U("203.0.113.45")),
    boost::optional<utility::string_t>(U("status:pending")),
    boost::optional<utility::string_t>(U("tenant:my-tenant-123")),
    boost::optional<utility::string_t>(U("cmt_987654321")),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentIdsResponse>> task){
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<ModerationAPIGetCommentIdsResponse>();
        (void)safeResp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]

---