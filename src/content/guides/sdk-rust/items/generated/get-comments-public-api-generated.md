
req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| page | i32 | No |  |
| direction | models::SortDirections | No |  |
| sso | String | No |  |
| skip | i32 | No |  |
| skip_children | i32 | No |  |
| limit | i32 | No |  |
| limit_children | i32 | No |  |
| count_children | bool | No |  |
| fetch_page_for_comment_id | String | No |  |
| include_config | bool | No |  |
| count_all | bool | No |  |
| includei10n | bool | No |  |
| locale | String | No |  |
| modules | String | No |  |
| is_crawler | bool | No |  |
| include_notification_count | bool | No |  |
| as_tree | bool | No |  |
| max_tree_depth | i32 | No |  |
| use_full_translation_ids | bool | No |  |
| parent_id | String | No |  |
| search_text | String | No |  |
| hash_tags | Vec<String> | No |  |
| user_id | String | No |  |
| custom_config_str | String | No |  |
| after_comment_id | String | No |  |
| before_comment_id | String | No |  |

## Response

Returns: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comments_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetCommentsPublicParams = GetCommentsPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    url_id: String::from("news/2025/politics-election-preview"),
    page: Some(1),
    direction: Some(models::SortDirections::Desc),
    limit: Some(25),
    skip: Some(0),
    include_config: Some(true),
    count_children: Some(false),
    hash_tags: Some(vec![String::from("breaking"), String::from("election")]),
    user_id: Some(String::from("user-4521")),
    after_comment_id: Some(String::from("c-9876")),
    include_notification_count: Some(true),
    as_tree: Some(true),
    max_tree_depth: Some(3),
    locale: Some(String::from("en-US")),
    ..Default::default()
};

let response: GetCommentsPublic200Response = get_comments_public(&configuration, params).await?;
[inline-code-end]
