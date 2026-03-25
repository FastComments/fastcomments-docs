## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_question_config Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run_delete_question_config() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteQuestionConfigParams = DeleteQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-config-8742".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_question_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---