## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| edit_key | String | Не |  |

## Отговор

Връща: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за delete_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_vote() -> Result<(), Error> {
    let params: DeleteVoteParams = DeleteVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/67890/comment/12345".to_string(),
        edit_key: Some("user-editkey-7f3b".to_string()),
    };
    let response: DeleteCommentVote200Response = delete_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---