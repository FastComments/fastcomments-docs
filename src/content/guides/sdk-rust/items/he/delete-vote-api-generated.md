## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| edit_key | String | לא |  |

## תגובה

מחזיר: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת delete_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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