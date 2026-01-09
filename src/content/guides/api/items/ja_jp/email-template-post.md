[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントはメールテンプレートを作成する機能を提供します。

注意:

- 同じドメイン内で同じ `emailTemplateId` を持つテンプレートを複数作成することはできません。
- しかしワイルドカードテンプレート（`domain` = `*`）と同じ `emailTemplateId` に対するドメイン固有のテンプレートを共存させることはできます。
- `domain` を指定するのは、異なるドメインごとにテンプレートを持ちたい場合や、テスト用に特定のテンプレートを使いたい場合（例：`domain` を `localhost` に設定）にのみ関連します。
- `domain` を指定する場合、その値は `DomainConfig` と一致している必要があります。エラー時には有効なドメインの一覧が提供されます。
- テンプレート構文は EJS で、レンダリングには 500ms のタイムアウトが設定されています。レンダリングの P99 は <5ms なので、500ms に到達する場合は何か問題があります。
- **保存するにはテンプレートが指定した `testData` でレンダリングされる必要があります。** レンダリングエラーは集約されダッシュボードで報告されます（API でまもなく利用可能になります）。

テンプレートを追加するために必要な最小データは次のとおりです:

[inline-code-attrs-start title = '最小 EmailTemplate POST cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

サイトごとにテンプレートを持ちたい場合は、`domain` を定義します:

[inline-code-attrs-start title = 'EmailTemplate POST cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 作成されたテンプレート。 **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]