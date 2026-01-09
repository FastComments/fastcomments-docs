[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

この API エンドポイントは、`id` によって `TenantPackage` を更新する機能を提供します。

`TenantPackage` を更新する際の制限は次の通りです：

- もし `hasFlexPricing` を true に設定する場合は、同じリクエスト内で全ての `flex*` パラメータが必須になります。
- `name` は `50 characters` を超えてはなりません。
- `forWhoText` の各項目は `200 characters` を超えてはなりません。
- `featureTaglines` の各項目は `100 characters` を超えてはなりません。
- `TenantPackage` は親テナントより「小さい」必要があります。例えば、すべての `max*` パラメータは親テナントより小さい値でなければなりません。 
- `TenantPackage` に関連付けられた `tenantId` を変更することはできません。

[inline-code-attrs-start title = 'TenantPackage PATCH cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]