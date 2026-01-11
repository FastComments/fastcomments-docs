req
tenantId
afterId

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| after_id | String | Не |  |
| limit | i32 | Не |  |
| tags | Vec<String> | Не |  |

## Отговор

Връща: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)