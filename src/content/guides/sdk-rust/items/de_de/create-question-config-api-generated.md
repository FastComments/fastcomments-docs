## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| create_question_config_body | models::CreateQuestionConfigBody | Ja |  |

## Antwort

Rückgabe: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_config_response.rs)

## Beispiel

[inline-code-attrs-start title = 'create_question_config Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = CreateQuestionConfigParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_question_config_body: models::CreateQuestionConfigBody {
        description: Some("Survey for news article feedback".to_string()),
        custom_options: Some(vec![
            QuestionConfigCustomOptionsInner {
                option_key: "allow_multiple".to_string(),
                option_value: "true".to_string(),
            },
        ]),
        ..Default::default()
    },
};

let response = create_question_config(&configuration, params).await?;
[inline-code-end]