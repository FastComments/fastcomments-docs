---
### 리소스 사용량

API에서 데이터를 가져오는 것이 계정 사용량에 포함된다는 점에 유의하십시오.

각 리소스는 해당 사용량을 자체 섹션에 나열합니다.

일부 리소스는 다른 리소스보다 제공 비용이 더 많이 듭니다. 각 엔드포인트는 API 호출당 정해진 크레딧 비용을 가지고 있습니다. 일부 엔드포인트의 경우 옵션 및 응답 크기에 따라 크레딧 수가 달라집니다.

API 사용량은 [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 페이지에서 확인할 수 있으며 몇 분마다 업데이트됩니다.

#### 주의!

Comment API에서 `urlId`에 전달할 값을 결정할 때 혼란을 줄이기 위해 먼저 Pages 문서를 읽어볼 것을 권장합니다.

### 웹훅

Webhook 구독에는 별도의 가이드가 있습니다. `POST`, `GET` 및 `DELETE /api/v1/webhooks`, 그리고 `GET /api/v1/webhooks/sample-payloads`는 [Managing Subscriptions via API](/guide-webhooks.html#webhooks-api-subscriptions)에서 문서화되어 있으며, 이벤트 페이로드는 [Webhook Structures](/guide-webhooks.html#webhooks-structures)에서 확인할 수 있습니다.

---