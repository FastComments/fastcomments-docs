[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

この API はユーザーに表示するためのコメントを取得するために使用されます。例えば、承認されていないコメントやスパムコメントを自動的にフィルタリングします。

### Pagination

ページネーションは、パフォーマンス要件やユースケースに応じて、次のいずれかの方法で行うことができます:

1. Fastest: **Precalculated Pagination**:
   1. これは、当社の事前構築されたウィジェットやクライアントを使用する場合に FastComments が動作する方法です。
   2. 「次へ」をクリックすると単にページ数が増加します。
   3. これはキー・バリュー ストアから取得されるように考えることができます。
   4. この方法では、`page` パラメータを `0` から開始して定義し、ソート方向を `direction` として指定します。
   5. ページサイズはカスタマイズルールで変更できます。
2. Most Flexible: **Flexible Pagination**:
   1. この方法ではカスタムの `limit` と `skip` パラメータを定義できます。`page` を渡さないでください。
   2. ソートの `direction` もサポートされています。
   3. `limit` は `skip` を適用した後に返される合計数です。
      - 例: `page size = 100` かつ `page = 2` の場合、`skip = 200, limit = 100` を設定します。
   4. 子コメントもページネーションにカウントされます。`asTree` オプションを使うことでこれを回避できます。
      - `limitChildren` と `skipChildren` により子のページネーションが可能です。
      - `maxTreeDepth` により返されるスレッドの深さを制限できます。

### Threads

1. `Precalculated Pagination` を使用する場合、コメントは *page* ごとにグループ化され、スレッド内のコメントは全体のページに影響します。
   1. この方法では、クライアントは `parentId` に基づいてスレッドを判断できます。
   2. 例えば、あるページにトップレベルのコメントが1件とその返信が29件あり、API に `page=0` を設定すると、トップレベルのコメント1件と29件の子コメントが返されます。
   3. [Example image here illustrating multiple pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. `Flexible Pagination` を使用する場合、`parentId` パラメータを定義できます。
   1. トップレベルのコメントのみを取得するにはこれを null に設定してください。
   2. スレッドを表示するには、再度 API を呼び出して `parentId` を渡します。
   3. 一般的な解決方法としては、トップレベルのコメント用に API 呼び出しを行い、その後各コメントの子コメントを取得するために並列の API 呼び出しを行う方法があります。
3. __新機能（2023年2月）!__ `&asTree=true` を使用してツリーとして取得できます。
   1. これは `Flexible Pagination as a Tree` と考えることができます。
   2. ページネーションにはトップレベルのコメントのみがカウントされます。
   3. ツリーをルートから開始するには `parentId=null` を設定してください（`parentId` を設定する必要があります）。
   4. ページネーションには `skip` と `limit` を設定してください。
   5. `asTree` を `true` に設定してください。
   6. このシナリオではバックエンドがより多くの処理を行うため、クレジットコストが `2x` に増加します。
   7. 必要に応じて `maxTreeDepth`、`limitChildren`、`skipChildren` を設定してください。

### Trees Explained

`asTree` を使用するとページネーションの考え方が分かりにくくなることがあります。参考になる図はこちらです:

<div class="screenshot white-bg">
    <div class="title">ツリーページネーション図</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="ツリーページネーション図" />
</div>

### Fetching Comments in The Context of a User

`/comments` API は、異なるユースケースに対して 2 つのコンテキストで使用できます:

- 自身のクライアントを構築するための情報でソートおよびタグ付けされたコメントを返す場合。
  - この場合は query パラメータに `contextUserId` を定義してください。
- カスタム統合のためにバックエンドからコメントを取得する場合。
  - `contextUserId` を指定しない場合、プラットフォームはこれをデフォルトとします。

[inline-code-attrs-start title = 'コメントの事前計算ページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'コメントの柔軟なページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'ユーザーコンテキストでのコメントの柔軟なページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'ユーザーコンテキストでのトップレベルコメントのみの柔軟なページネーション'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

コメントをツリーとして返すことが可能で、ページネーションはトップレベルのコメントのみをカウントします。

[inline-code-attrs-start title = 'ユーザーコンテキストでのツリー形式のコメント'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

トップレベルのコメントとその直下の子のみを取得したいですか? こうする方法の一例を示します:

[inline-code-attrs-start title = '最大深度付きツリー形式のコメント'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

ただし、UI 上では各コメントに「返信を表示」ボタンを表示するかどうかを知る必要がある場合があります。ツリーとしてコメントを取得すると、該当する場合に `hasChildren` プロパティがコメントに付与されます。

### Get Comments as a Tree, Searching by Hash Tag

API を使用してハッシュタグで検索することが可能で、テナント全体（1 ページや `urlId` に限定されません）を横断して検索できます。

この例では `urlId` を省略し、複数のハッシュタグで検索します。API は要求されたすべてのハッシュタグを持つコメントのみを返します。

[inline-code-attrs-start title = 'ハッシュタグによるユーザーコンテキストでのツリー形式のコメント'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'コメントリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'コメントレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

おそらく `Comment` API を `urlId` パラメータと一緒に使用したいでしょう。まず `Pages` API を呼び出して、利用可能な `urlId` の値がどのようになっているかを確認できます。

#### Anonymous Actions

匿名でコメントする場合、コメントの取得時やフラグ付け・ブロックを行う際に `anonUserId` を渡すことをお勧めします。

(!) これは多くのアプリストアで必須です。ユーザーはログインしていなくても、自分が見えるユーザー生成コンテンツを通報できる必要があります。これを行わないと、アプリが当該ストアから削除される可能性があります。

#### Comments Not Being Returned

コメントが承認されており、スパムではないことを確認してください。

---