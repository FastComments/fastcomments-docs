문서를 그룹화(groupBy가 제공된 경우)하고 여러 연산을 적용하여 집계합니다. 다양한 연산(예: sum, countDistinct, avg 등)을 지원합니다.

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| aggregation_request | models::AggregationRequest | 예 |  |
| parent_tenant_id | String | 아니요 |  |
| include_stats | bool | 아니요 |  |

## 응답

반환: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---