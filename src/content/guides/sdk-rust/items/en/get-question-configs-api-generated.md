## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | f64 | No |  |

## Response

Returns: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_question_configs Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetQuestionConfigsParams = GetQuestionConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetQuestionConfigs200Response = get_question_configs(&configuration, params).await?;
    let _cfgs: GetQuestionConfigs200Response = response;
    Ok(())
}
[inline-code-end]
