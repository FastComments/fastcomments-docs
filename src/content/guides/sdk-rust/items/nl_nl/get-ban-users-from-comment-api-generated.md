## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| sso | String | Nee |  |

## Respons

Geeft terug: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_from_comment_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_ban_users_from_comment Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_banned_users_from_comment() -> Result<GetBannedUsersFromCommentResponse, Error> {
    let params: GetBanUsersFromCommentParams = GetBanUsersFromCommentParams {
        comment_id: String::from("news/tech/acme-launch/comment-42"),
        sso: Some(String::from("acme-corp-sso-token-2026-06")),
    };
    let response: GetBannedUsersFromCommentResponse =
        get_ban_users_from_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---