[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントは `id` によって `Tenant` を更新する機能を提供します。

`Tenant` の更新には以下の制限があります：

- 次の値は更新できません：
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` は未来日には設定できません。
- `name` は `200 characters` を超えてはいけません。
- `email` は `300 characters` を超えてはいけません。
- `email` は FastComments.com のすべてのテナント間で一意でなければなりません。
- `billingInfoValid` を `true` に設定する場合、同じリクエスト内で `billingInfo` を提供する必要があります。
- 自身のテナントに紐付いた `packageId` を更新することはできません。
- 自身のテナントに紐付いた `paymentFrequency` を更新することはできません。

[inline-code-attrs-start title = 'Tenant PATCH cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]