## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| adjust_comment_votes_params | models::AdjustCommentVotesParams | Да |  |
| broadcast_id | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/adjust_votes_response.rs)

## Пример

[inline-code-attrs-start title = 'post_adjust_comment_votes Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn adjust_votes_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostAdjustCommentVotesParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "comment-9876".to_string(),
        adjust_comment_votes_params: models::AdjustCommentVotesParams::default(),
        broadcast_id: Some("broadcast-2023-11".to_string()),
        sso: Some("sso-xyz".to_string()),
    };
    let _response = post_adjust_comment_votes(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---