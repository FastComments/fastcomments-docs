## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_feed_post_params | models::CreateFeedPostParams | 是 |  |
| broadcast_id | String | 否 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| skip_dup_check | bool | 否 |  |

## 响应

返回: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---