[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `Tenant`를 추가하는 기능을 제공합니다.

`Tenant`를 생성할 때 다음과 같은 제한이 있습니다:

- `name`은 필수입니다.
- `domainConfiguration`은 필수입니다.
- `Tenant`를 생성할 때 다음 값들은 제공할 수 없습니다:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- `signUpDate`는 미래 날짜일 수 없습니다.
- `name`은 `200 characters`를 초과할 수 없습니다.
- `email`은 `300 characters`를 초과할 수 없습니다.
- `email`은 FastComments.com의 모든 테넌트에서 고유해야 합니다.
- 부모 테넌트에 유효한 `TenantPackage`가 정의되어 있지 않으면 테넌트를 생성할 수 없습니다.
  - 테넌트가 FastComments.com을 통해 생성된 경우 이는 문제가 되지 않습니다.
- 패키지에 정의된 `maxWhiteLabeledTenants`보다 많은 테넌트를 생성할 수 없습니다.
- 화이트 라벨링이 활성화된 `parent tenant`의 id인 `tenantId` 쿼리 파라미터를 반드시 지정해야 합니다.

우리는 몇 가지 파라미터만으로 `Tenant`를 생성할 수 있습니다:

[inline-code-attrs-start title = '테넌트 생성 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = '테넌트 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '테넌트 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenant?: Tenant; // 성공 시 생성된 tenant 전체를 반환합니다.
}
[inline-code-end]

---