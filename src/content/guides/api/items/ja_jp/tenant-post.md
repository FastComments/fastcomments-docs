[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

このルートは単一の `Tenant` を追加する機能を提供します。

`Tenant` を作成する際の制約は以下の通りです:

- `name` は必須です。
- `domainConfiguration` は必須です。
- `Tenant` を作成する際に以下の値を指定してはいけません:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` は未来の日付にできません。
- `name` は `200 characters` を超えてはいけません。
- `email` は `300 characters` を超えてはいけません。
- `email` は FastComments.com の全テナントで一意でなければなりません。
- 親テナントに有効な `TenantPackage` が定義されていない場合、テナントを作成できません。
  - もしあなたのテナントが FastComments.com 経由で作成されているなら、この問題は発生しないはずです。
- パッケージで定義されている `maxWhiteLabeledTenants` より多くのテナントを作成することはできません。
- white labeling が有効な `parent tenant` の ID である `tenantId` クエリパラメータを指定する必要があります。

少数のパラメータのみで `Tenant` を作成できます:

[inline-code-attrs-start title = 'Tenant 作成 cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Tenant 作成リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 作成レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** 失敗時に含まれます。 **/
    reason?: string
    tenant?: Tenant; // 成功時に作成された完全な tenant を返します。
}
[inline-code-end]

---