[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

このルートは単一の `Moderator` を招待する機能を提供します。

招待メールを `Moderator` に送信するには次の制限があります:
- `Moderator` は既に存在している必要があります。
- `fromName` は `100 characters` を超えてはいけません。

**注意:**
- 指定したメールアドレスのユーザーが既に存在する場合、そのユーザーはテナントのコメントをモデレートするよう招待されます。
- 指定したメールアドレスのユーザーが **存在しない場合**、招待リンクはアカウント作成の手順へ案内します。
- 招待は `30 days` 後に期限切れになります。

メールアドレスだけしか知らないユーザーのために `Moderator` を作成できます:

[inline-code-attrs-start title = 'Moderator 招待の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

これにより `Bob at TenantName is inviting you to be a moderator...` のようなメールが送信されます。

[inline-code-attrs-start title = 'Moderator 招待リクエストの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** ユーザーに送信されるメールはこの名前から送信されたように表示されます。 **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 招待レスポンスの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---