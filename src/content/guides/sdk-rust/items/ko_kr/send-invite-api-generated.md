---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| from_name | String | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'send_invite 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: SendInviteParams = SendInviteParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "articles/2026/01/ai-news-12345".to_string(),
    from_name: "Acme Newsroom".to_string(),
    reply_to: Some("editorial@acme.com".to_string()),
    message: Some("You have been invited to moderate comments on this article.".to_string()),
};

let invite_response: FlagCommentPublic200Response = send_invite(&configuration, params).await?;
[inline-code-end]

---