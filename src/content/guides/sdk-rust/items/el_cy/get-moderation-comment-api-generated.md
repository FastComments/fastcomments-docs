## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| comment_id | String | Ναι |  |
| include_email | bool | Όχι |  |
| include_ip | bool | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_moderation_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment() -> Result<ModerationApiCommentResponse, Error> {
    let params: GetModerationCommentParams = GetModerationCommentParams {
        comment_id: String::from("cmt-48291"),
        include_email: Some(true),
        include_ip: Some(false),
        sso: Some(String::from("sso-acme-corp-2026-token")),
    };
    let response: ModerationApiCommentResponse = get_moderation_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---