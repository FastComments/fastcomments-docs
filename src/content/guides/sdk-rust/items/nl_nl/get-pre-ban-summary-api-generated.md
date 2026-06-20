## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| include_by_user_id_and_email | bool | Nee |  |
| include_by_ip | bool | Nee |  |
| include_by_email_domain | bool | Nee |  |
| sso | String | Nee |  |

## Respons

Geeft terug: [`PreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pre_ban_summary.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_pre_ban_summary Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPreBanSummaryParams = GetPreBanSummaryParams {
    comment_id: String::from("news/article-9876-comment-42"),
    include_by_user_id_and_email: Some(true),
    include_by_ip: Some(false),
    include_by_email_domain: Some(true),
    sso: Some(String::from("sso-acme-corp-2026")),
};
let pre_ban_summary: PreBanSummary = get_pre_ban_summary(configuration, params).await?;
[inline-code-end]