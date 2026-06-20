## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| url_id | String | Evet |  |
| broadcast_id | String | Evet |  |
| vote_body_params | models::VoteBodyParams | Evet |  |
| session_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Örnek

[inline-code-attrs-start title = 'vote_comment Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: VoteCommentParams = VoteCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt_8392a1".to_string(),
    url_id: "news/article-2026-06-19-rust-release".to_string(),
    broadcast_id: "broadcast_2026_06".to_string(),
    vote_body_params: models::VoteBodyParams { value: 1 },
    session_id: Some("sess_4f9b2c".to_string()),
    sso: Some("sso_token_abcd1234".to_string()),
};
let response: VoteResponse = vote_comment(&configuration, params).await?;
[inline-code-end]