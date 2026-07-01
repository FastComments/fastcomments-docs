## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| render_email_template_body | models::RenderEmailTemplateBody | Ναι |  |
| locale | String | Όχι |  |

## Response

Επιστρέφει: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_response.rs)

## Example

[inline-code-attrs-start title = 'Παράδειγμα render_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let mut vars = std::collections::HashMap::new();
vars.insert("article_title".to_string(), "Breaking News".to_string());
vars.insert("author".to_string(), "Jane Smith".to_string());

let body = models::RenderEmailTemplateBody {
    template_id: "newsletter".to_string(),
    variables: vars,
};

let params = RenderEmailTemplateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    render_email_template_body: body,
    locale: Some("en-US".to_string()),
};

let response = render_email_template(&configuration, params).await?;
[inline-code-end]

---