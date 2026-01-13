---
req
tenantId
afterId

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| after_id | String | Ні |  |
| limit | i32 | Ні |  |
| tags | Vec<String> | Ні |  |
| sso | String | Ні |  |
| is_crawler | bool | Ні |  |
| include_user_info | bool | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---