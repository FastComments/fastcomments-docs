## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Да |  |
| context_user_id | String | Не |  |
| do_spam_check | bool | Не |  |
| is_live | bool | Не |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---