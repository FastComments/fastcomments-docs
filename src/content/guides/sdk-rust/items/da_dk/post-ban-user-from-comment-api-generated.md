## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| ban_email | bool | Nej |  |
| ban_email_domain | bool | Nej |  |
| ban_ip | bool | Nej |  |
| delete_all_users_comments | bool | Nej |  |
| banned_until | String | Nej |  |
| is_shadow_ban | bool | Nej |  |
| update_id | String | Nej |  |
| ban_reason | String | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Eksempel

[inline-code-attrs-start title = 'post_ban_user_from_comment Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostBanUserFromCommentParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "cmt-12345".to_string(),
        ban_email: Some(true),
        ban_email_domain: Some(false),
        ban_ip: Some(true),
        delete_all_users_comments: Some(false),
        banned_until: Some("2024-12-31T23:59:59Z".to_string()),
        is_shadow_ban: Some(false),
        update_id: Some("upd-987".to_string()),
        ban_reason: Some("spam".to_string()),
        sso: Some("sso-provider".to_string()),
    };
    let _result: BanUserFromCommentResult = post_ban_user_from_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]