문서를 그룹화하여 (groupBy가 제공된 경우) 여러 연산을 적용해 집계합니다.
다양한 연산(예: sum, countDistinct, avg 등)이 지원됩니다.

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| aggregationRequest | AggregationRequest | 예 |  |
| parentTenantId | string | 아니요 |  |
| includeStats | boolean | 아니요 |  |

## 응답

반환: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---