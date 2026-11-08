दस्तावेज़ों को समूहित करके (यदि groupBy दिया गया है) एकत्रित करता है और कई ऑपरेशन लागू करता है।
विभिन्न ऑपरेशन (जैसे sum, countDistinct, avg, आदि) समर्थित हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| aggregation_request | models::AggregationRequest | हाँ |  |
| parent_tenant_id | String | नहीं |  |
| include_stats | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'aggregate उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let aggregation_request = models::AggregationRequest::default();
    let params = AggregateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        aggregation_request,
        parent_tenant_id: Some("global-tenant".to_string()),
        include_stats: Some(true),
    };
    let _response = aggregate(&config, params).await?;
    Ok(())
}
[inline-code-end]