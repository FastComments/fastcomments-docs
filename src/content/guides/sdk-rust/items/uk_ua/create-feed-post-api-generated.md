## Параметри

| Назва | Type | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| create_feed_post_params | models::CreateFeedPostParams | Так |  |
| broadcast_id | String | Ні |  |
| is_live | bool | Ні |  |
| do_spam_check | bool | Ні |  |
| skip_dup_check | bool | Ні |  |

## Відповідь

Повертає: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---