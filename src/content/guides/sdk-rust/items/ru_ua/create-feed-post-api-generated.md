## Параметры

| Name | Type | Обязательно | Описание |
|------|------|------------|----------|
| tenant_id | String | Да |  |
| create_feed_post_params | models::CreateFeedPostParams | Да |  |
| broadcast_id | String | Нет |  |
| is_live | bool | Нет |  |
| do_spam_check | bool | Нет |  |
| skip_dup_check | bool | Нет |  |

## Ответ

Возвращает: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---