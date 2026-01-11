## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_feed_post_params | models::CreateFeedPostParams | Да |  |
| broadcast_id | String | Не |  |
| is_live | bool | Не |  |
| do_spam_check | bool | Не |  |
| skip_dup_check | bool | Не |  |

## Одговор

Враћа: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---