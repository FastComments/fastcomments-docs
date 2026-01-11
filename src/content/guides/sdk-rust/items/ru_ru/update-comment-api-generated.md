## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Да |  |
| context_user_id | String | Нет |  |
| do_spam_check | bool | Нет |  |
| is_live | bool | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---