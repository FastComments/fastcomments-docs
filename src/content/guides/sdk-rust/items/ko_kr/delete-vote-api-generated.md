## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| edit_key | String | 아니오 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_vote 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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