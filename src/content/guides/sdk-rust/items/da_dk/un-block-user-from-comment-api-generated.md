## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| un_block_from_comment_params | models::UnBlockFromCommentParams | Ja |  |
| user_id | String | Nej |  |
| anon_user_id | String | Nej |  |

## Svar

Returnerer: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Eksempel

[inline-code-attrs-start title = 'un_block_user_from_comment Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnBlockUserFromCommentParams = UnBlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/comments/42".to_string(),
        un_block_from_comment_params: models::UnBlockFromCommentParams {
            reason: Some("mistaken moderation".to_string()),
            unblock_children: Some(true),
        },
        user_id: Some("user-12345".to_string()),
        anon_user_id: None,
    };
    let success: UnblockSuccess = un_block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---