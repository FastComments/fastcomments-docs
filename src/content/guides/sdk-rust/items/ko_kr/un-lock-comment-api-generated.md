## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`LockComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/lock_comment_200_response.rs)

## 예제

[inline-code-attrs-start title = 'un_lock_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnLockCommentParams = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-987654321".to_string(),
        broadcast_id: "news/world-update-2026-04-25".to_string(),
        sso: Some("sso-token-abcdef123456".to_string()),
    };
    let response: LockComment200Response = un_lock_comment(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---