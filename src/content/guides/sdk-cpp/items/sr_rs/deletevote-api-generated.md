## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| editKey | string | Не |  |

## Одговор

Враћа: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Пример

[inline-code-attrs-start title = 'deleteVote Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("vote-987654321");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("edit-key-abc123"));

api->deleteVote(tenantId, id, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<VoteDeleteResponse>();
        (void)finalResp;
    } catch (...) {
        auto errorResp = std::make_shared<VoteDeleteResponse>();
        (void)errorResp;
    }
});
[inline-code-end]

---