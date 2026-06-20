## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| edit_key | String | Ні |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteVoteParams = DeleteVoteParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("article-5678-comment-1234"),
        edit_key: Some(String::from("editkey-9b2f4e")),
    };
    let response: VoteDeleteResponse = delete_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---