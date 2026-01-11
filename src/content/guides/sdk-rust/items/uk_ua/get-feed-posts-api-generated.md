---
req
tenantId
afterId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| after_id | String | Ні |  |
| limit | i32 | Ні |  |
| tags | Vec<String> | Ні |  |

## Відповідь

Повертає: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

---