req
tenantId
afterId

## Параметры

| Название | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| after_id | String | No |  |
| limit | i32 | No |  |
| tags | Vec<String> | No |  |
| sso | String | No |  |
| is_crawler | bool | No |  |
| include_user_info | bool | No |  |

## Ответ

Возвращает: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---