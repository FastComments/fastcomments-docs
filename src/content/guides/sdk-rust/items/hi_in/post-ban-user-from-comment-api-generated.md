## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| ban_email | bool | नहीं |  |
| ban_email_domain | bool | नहीं |  |
| ban_ip | bool | नहीं |  |
| delete_all_users_comments | bool | नहीं |  |
| banned_until | String | नहीं |  |
| is_shadow_ban | bool | नहीं |  |
| update_id | String | नहीं |  |
| ban_reason | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/ban_user_from_comment_result.rs)

## उदाहरण

[inline-code-attrs-start title = 'post_ban_user_from_comment उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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