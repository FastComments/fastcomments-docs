[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントは、idと更新する属性のみを指定することで、メールテンプレートを更新する機能を提供します。

テンプレート作成時と同じ検証が全て適用される点に注意してください。例えば:

- テンプレートはレンダー可能でなければなりません。これは各更新時に検査されます。
- 同一ドメインに対して重複したテンプレートを持つことはできません（そうすると片方が黙って無視されます）。

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 更新されたメールテンプレート。 **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---