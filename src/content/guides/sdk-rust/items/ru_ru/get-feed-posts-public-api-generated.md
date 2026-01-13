---
req
tenantId
afterId

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| after_id | String | Нет |  |
| limit | i32 | Нет |  |
| tags | Vec<String> | Нет |  |
| sso | String | Нет |  |
| is_crawler | bool | Нет |  |
| include_user_info | bool | Нет |  |

## Ответ

Возвращает: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---