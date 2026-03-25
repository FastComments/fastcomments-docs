## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| update_question_config_body | models::UpdateQuestionConfigBody | Oui |  |

## Réponse

Renvoie : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_question_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateQuestionConfigParams = UpdateQuestionConfigParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "news/article-2026-03-readability".to_string(),
    update_question_config_body: models::UpdateQuestionConfigBody {
        question_text: Some("Was this article helpful?".to_string()),
        required: Some(true),
        rendering_type: Some(models::QuestionRenderingType::Inline),
        custom_options: Some(vec![
            models::QuestionConfigCustomOptionsInner { label: "Very helpful".to_string(), value: "very_helpful".to_string() },
            models::QuestionConfigCustomOptionsInner { label: "Somewhat helpful".to_string(), value: "somewhat_helpful".to_string() },
            models::QuestionConfigCustomOptionsInner { label: "Not helpful".to_string(), value: "not_helpful".to_string() },
        ]),
        when_save: Some(models::QuestionWhenSave::AskOnSave),
        ..Default::default()
    },
};
let response: FlagCommentPublic200Response = update_question_config(&configuration, params).await?;
[inline-code-end]

---