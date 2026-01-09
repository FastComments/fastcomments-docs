[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

このルートは、IDによって単一の `DomainConfig` を削除します。

- 注意：`DomainConfig` を削除すると、そのドメインは FastComments の使用が許可されなくなります。
- 注意：UIからドメインを再追加すると、オブジェクトが再作成されます（`domain` のみが設定されます）。

[inline-code-attrs-start title = 'DomainConfig 削除の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig 削除リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig 削除レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---