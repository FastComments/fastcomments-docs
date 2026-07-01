## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| ban_email | bool | Non |  |
| ban_email_domain | bool | Non |  |
| ban_ip | bool | Non |  |
| delete_all_users_comments | bool | Non |  |
| banned_until | String | Non |  |
| is_shadow_ban | bool | Non |  |
| update_id | String | Non |  |
| ban_reason | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_ban_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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