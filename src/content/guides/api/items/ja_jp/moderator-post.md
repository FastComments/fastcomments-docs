[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

このルートは単一の `Moderator` を追加する機能を提供します。

`Moderator` を作成する際の制限は次の通りです：

- `name` と `email` は常に提供する必要があります。`userId` は任意です。
- Moderator を作成する際、次の値は指定できません：
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- `userId` が指定されている場合、そのユーザーは存在していなければなりません。
- `userId` が指定されている場合、そのユーザーはクエリパラメータで指定されたのと同じ `tenantId` に属している必要があります。
- 同じテナント内で、同一の `email` を持つモデレーターを二人追加することはできません。

メールアドレスのみが分かっているユーザーのために `Moderator` を作成できます：

[inline-code-attrs-start title = 'メールでの Moderator 作成 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

または、当社のテナントに所属するユーザーのモデレーション統計を追跡するために `Moderator` を作成することもできます：

[inline-code-attrs-start title = 'テナントユーザー経由での Moderator 作成 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 作成リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 作成レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** 失敗時に含まれます。 **/
    reason?: string
    moderator?: Moderator; // 成功時には作成されたモデレーター全体を返します。
}
[inline-code-end]

---