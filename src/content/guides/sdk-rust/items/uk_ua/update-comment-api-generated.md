## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | Так |  |
| context_user_id | String | Ні |  |
| do_spam_check | bool | Ні |  |
| is_live | bool | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---