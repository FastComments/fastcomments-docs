[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 `id` 로 `Tenant` 를 업데이트할 수 있는 기능을 제공합니다.

`Tenant` 를 업데이트할 때는 다음 제한 사항이 있습니다:

- 다음 값은 업데이트할 수 없습니다:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` 은 미래일 수 없습니다.
- `name` 은 `200 characters` 보다 길 수 없습니다.
- `email` 은 `300 characters` 보다 길 수 없습니다.
- `email` 은 FastComments.com 모든 테넌트에서 고유해야 합니다.
- `billingInfoValid` 를 `true` 로 설정할 때, 동일한 요청에 `billingInfo` 를 제공해야 합니다.
- 자신의 테넌트와 연결된 `packageId` 를 업데이트할 수 없습니다.
- 자신의 테넌트와 연결된 `paymentFrequency` 를 업데이트할 수 없습니다.

[inline-code-attrs-start title = '테넌트 PATCH cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = '테넌트 PATCH 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '테넌트 PATCH 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---