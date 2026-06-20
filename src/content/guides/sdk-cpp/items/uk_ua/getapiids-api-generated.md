## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filters | string | Ні |  |
| searchFilters | string | Ні |  |
| afterId | string | Ні |  |
| demo | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getApiIds'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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