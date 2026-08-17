[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

この API は、ユーザーに表示するためのコメントを取得するために使用されます。たとえば、未承認またはスパムコメントを自動的に除外します。

### ページネーション

ページネーションは、パフォーマンス要件とユースケースに応じて、次の 2 つの方法のいずれかで実行できます。

1. 最速: **事前計算ページネーション**:
   1. これは、事前に構築されたウィジェットとクライアントを使用する際の FastComments の動作方式です。
   2. 「次へ」をクリックすると、単にページ番号が増えます。
   3. これはキー・バリュー・ストアから取得されるものと考えてください。
   4. この方法では、`page` パラメータを `0` から開始し、ソート方向を `direction` として定義するだけです。
   5. ページサイズはカスタマイズルールで調整できます。
2. 最も柔軟: **柔軟なページネーション**:
   1. この方法では、カスタムの `limit` と `skip` パラメータを定義できます。`page` は渡さないでください。
   2. ソート `direction` もサポートされています。
   3. `limit` は、`skip` が適用された後に返す総数です。
      - 例: `page size = 100` で `page = 2` の場合、`skip = 200, limit = 100` と設定します。
   4. 子コメントもページネーションにカウントされます。`asTree` オプションを使用して回避できます。
      - `limitChildren` と `skipChildren` で子コメントをページネーションできます。
      - `maxTreeDepth` で返されるスレッドの深さを制限できます。

### スレッド

1. `Precalculated Pagination` を使用する場合、コメントは *ページ* ごとにグループ化され、スレッド内のコメントは全体のページに影響します。
   1. この方法では、`parentId` に基づいてクライアント側でスレッドを判定できます。
   2. 例として、トップレベルコメントが 1 件で 29 件の返信があるページで、API に `page=0` を設定すると、トップレベルコメントと 29 件の子コメントだけが取得されます。
2. `Flexible Pagination` を使用する場合、`parentId` パラメータを定義できます。
   1. これを null に設定すると、トップレベルコメントのみが取得されます。
   2. スレッドを表示するには、API を再度呼び出し、`parentId` を渡します。
   3. 一般的な解決策は、トップレベルコメント用に API 呼び出しを行い、続いて各コメントの子コメントを取得するために並列で API 呼び出しを行うことです。
3. __NEW 2023年2月から！__ `&asTree=true` を使用してツリーとして取得します。
   1. これは `Flexible Pagination as a Tree` と考えてください。
   2. ページネーションではトップレベルコメントのみがカウントされます。
   3. `parentId=null` を設定してツリーをルートから開始します（`parentId` を設定する必要があります）。
   4. ページネーションのために `skip` と `limit` を設定します。
   5. `asTree` を `true` に設定します。
   6. このシナリオではバックエンドの作業が大幅に増えるため、クレジットコストが `2x` に増加します。
   7. 必要に応じて `maxTreeDepth`、`limitChildren`、`skipChildren` を設定します。

### ツリーの説明

`asTree` を使用する場合、ページネーションの考え方が難しいことがあります。便利な図をご覧ください。

<div class="screenshot white-bg">
    <div class="title">ツリーページネーション図</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="ツリーページネーション図" />
</div>

### ユーザーコンテキストでのコメント取得

`/comments` API は、異なるユースケースに応じて 2 つのコンテキストで使用できます。

- 独自クライアント構築のために、ソートされ情報がタグ付けされたコメントを返す場合。
  - この場合、`contextUserId` クエリパラメータを定義します。
- カスタム統合のためにバックエンドからコメントを取得する場合。
  - プラットフォームは `contextUserId` がなくてもデフォルトでこの方式になります。

[inline-code-attrs-start title = 'コメント 事前計算ページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'コメント 柔軟なページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'ユーザーコンテキストでのコメント 柔軟なページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'トップレベルコメントのみのユーザーコンテキストでのコメント 柔軟なページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### ツリー形式のコメント取得

コメントをツリーとして返すことが可能で、ページネーションはトップレベルコメントのみをカウントします。

[inline-code-attrs-start title = 'ユーザーコンテキストでのコメント ツリー形式'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

トップレベルコメントとその直下の子コメントだけを取得したいですか？以下の方法があります。

[inline-code-attrs-start title = '最大深さ付きツリー形式のコメント'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

ただし、UI では各コメントに「返信を表示」ボタンを表示すべきかどうかを知る必要があるかもしれません。ツリーでコメントを取得する場合、該当するコメントには `hasChildren` プロパティが付与されます。

### ハッシュタグで検索するツリー形式のコメント取得

API を使用してハッシュタグで検索することが可能です。テナント全体（特定のページや `urlId` に限定されません）で検索できます。

この例では `urlId` を省略し、複数のハッシュタグで検索します。API は要求されたすべてのハッシュタグを持つコメントのみを返します。

[inline-code-attrs-start title = 'ハッシュタグで検索するユーザーコンテキストのツリー形式コメント'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### すべてのリクエストパラメータ

[inline-code-attrs-start title = 'コメント リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** コメントが関連付けられる urlId（ページ URL または記事 ID）。 **/
    urlId?: string
    /** このユーザーが返すコメントを制限します。 **/
    userId?: string
    /** ハッシュタグで検索する際に使用します。複数ハッシュタグの交差を絞り込むには、&hashTag=a&hashTag=b のように指定します。 **/
    hashTag?: string
    /** ソート方向。デフォルトは MR（最も関連性が高い）。他のオプションは OF（古い順） と NF（新しい順）。 **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 事前計算ページネーション: 取得するページ番号、0 から開始。すべてのコメント（最大 250 件）を取得するには -1 を指定します。 **/
    page?: number
    /** 柔軟なページネーション: 返すコメント数を指定します。 **/
    limit?: number
    /** 柔軟なページネーション: 各親に対して返す子コメント数を指定します。 **/
    limitChildren?: number
    /** 柔軟なページネーション: スキップするコメント数を指定します。 **/
    skip?: number
    /** 柔軟なページネーション: 各親に対してスキップする子コメント数を指定します。 **/
    skipChildren?: number
    /** ブロックおよびフラグ付けされたコメントを判定するために使用します。 **/
    contextUserId?: string
    /** ブロックおよびフラグ付けされたコメントを判定するために使用します。 **/
    anonUserId?: string
    /** 子コメントを取得するために使用します。 **/
    parentId?: string
    /** ツリー形式で取得するために使用します。 **/
    asTree?: boolean
    /** ツリーのどの深さまでデータを返すかを指定します。0 は子を返さず、1 は直下の子を返す、というように指定します。 **/
    maxTreeDepth?: number
}
[inline-code-end]

### レスポンス

[inline-code-attrs-start title = 'コメント レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** コメント一覧！ **/
    comments: Comment[]
}
[inline-code-end]

### 便利なヒント

#### URL ID

おそらく `Comment` API を `urlId` パラメータと共に使用したいでしょう。まず `Pages` API を呼び出すことで、利用可能な `urlId` の値がどのようなものか確認できます。 

#### 匿名アクション

匿名コメントの場合、コメント取得時やフラグ付け・ブロック操作時に `anonUserId` を渡すことが望ましいでしょう。

(!) これは多くのアプリストアで必須です。ユーザーはログインしていなくても閲覧できるユーザー生成コンテンツにフラグを付けられる必要があります。これを行わないと、アプリが該当ストアから削除される可能性があります。

#### コメントが返されない場合

コメントが承認済みであり、スパムでないことを確認してください。

---