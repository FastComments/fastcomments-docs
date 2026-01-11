## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_comment_params | models::CreateCommentParams | Да |  |
| is_live | bool | Нет |  |
| do_spam_check | bool | Нет |  |
| send_emails | bool | Нет |  |
| populate_notifications | bool | Нет |  |

## Ответ

Возвращает: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---