FastComments は OAuth 2.1 認可サーバーです。アプリケーションは 1 つの FastComments アカウントに紐付いたトークンを取得し、このガイドのすべてのエンドポイントで API キーの代わりに使用できます。これが Zapier アプリ、MCP サーバー、その他のサードパーティ統合が接続する方法です。

トークンは PKCE を使用した認可コードフローで発行されます。クライアントクレデンシャルやインプリシットグラントはありません。

### Discovery

エンドポイントの場所、サポートされているグラント、認証方法は標準メタデータ URL で公開されています:

[inline-code-attrs-start title = '認可サーバーメタデータ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

記載されているエンドポイント:

[inline-code-attrs-start title = 'OAuth エンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU リージョンのアカウントは `https://eu.fastcomments.com` を発行元として使用し、パスは同じです。

### Registering a client

クライアントはフローを開始する前に `client_id` と登録済みの `redirect_uri` が必要です。取得方法は 2 通りあります。

- **Dynamic Client Registration.** `POST /oauth/register` を RFC 7591 に従った JSON ボディ（`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`）で送信します。レスポンスに `client_id` と、機密クライアントの場合は `client_secret` が含まれます。登録は認証なしで、IP ごとにレート制限があります。
- **Client ID Metadata Document.** クライアントは自分が管理する `https` URL を `client_id` として使用します。FastComments がその URL を取得し、同じメタデータフィールドを読み取ります。登録呼び出しは不要です。

Zapier など FastComments ダッシュボードに一覧表示されているパートナーアプリは FastComments が直接登録します。マーケットプレイスのリスティングを構築し、ファーストパーティクライアントが必要な場合はサポートにお問い合わせください。

### Scopes

[inline-code-attrs-start title = 'スコープ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

スコープを指定しないリクエストは両方が付与されます。ユーザーは同意ページで要求されたスコープを確認できます。これら 2 つ以外のスコープを要求すると `invalid_scope` で失敗します。

### Step 1 - Authorization request

ユーザーのブラウザを認可エンドポイントへ送ります。すべてのクライアントで `S256` メソッドの PKCE が必須です。

[inline-code-attrs-start title = '認可リクエスト'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = '認可リクエストパラメータ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** クライアントが登録したリダイレクトURIのいずれかと正確に一致する必要があります。 **/
    redirect_uri: string
    /** スペースで区切ります。省略すると両方のスコープを要求します。 **/
    scope?: 'read' | 'write' | 'read write'
    /** リダイレクト時に変更されずに返されます。フローを開始したセッションにコールバックを結び付けるために使用します。 **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** オプションの RFC 8707 リソースインジケータ。送信された場合、トークンエンドポイントにも同じ値を送信する必要があります。 **/
    resource?: string
}
[inline-code-end]

ユーザーは必要に応じて FastComments にサインインし、アプリケーション名、接続されるアカウント、要求されたスコープが表示された同意ページを確認します。ユーザーはそのアカウントで **API Admin** 権限を保持している必要があります。保持していない場合は同意フォームの代わりに権限エラーが表示されます。承認するとブラウザは `redirect_uri` に `code` と `state` を付けてリダイレクトします。拒否すると `error=access_denied` が付いてリダイレクトされます。

認可コードは有効期限が 10 分で、1 回だけ交換できます。同じコードを 2 回目に交換すると、最初の交換で発行されたすべてのトークンが失効します。

### Step 2 - Token request

コードをトークンに交換します。ボディはフォームエンコードです。機密クライアントは `client_secret_basic`（HTTP Basic）または `client_secret_post`（ボディ内のシークレット）で認証します。パブリッククライアントは `client_id` のみ送信します。

[inline-code-attrs-start title = 'トークンリクエスト cURL 例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'トークンリクエストボディ (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** 機密クライアントのみ。代わりに HTTP Basic 認証として送信することもできます。 **/
    client_secret?: string
    code: string
    code_verifier: string
    /** 送信時は認可リクエストと一致する必要があります。 **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'トークンレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** プレフィックス fcat_。有効期限は1時間です。 **/
    access_token: string
    token_type: 'bearer'
    /** アクセストークンが期限切れになるまでの秒数。3600。 **/
    expires_in: number
    /** プレフィックス fcrt_。発行から30日間有効です。 **/
    refresh_token: string
    /** スペースで区切られた付与されたスコープ。 **/
    scope: string
}
[inline-code-end]

エラーは RFC 6749 に従います。`error` と `error_description` を含む JSON ボディが返され、`invalid_request`、`invalid_grant`、`invalid_scope`、`invalid_target`、`unsupported_grant_type` は HTTP 400、`invalid_client` は HTTP 401、レート制限時は HTTP 429 が返されます。

### Step 3 - Calling the API

アクセストークンをベアラートークンとして送信します。テナントはトークンに暗黙的に含まれるため `tenantId` はオプションです。指定した場合はトークンと一致しなければリクエストは失敗します。

[inline-code-attrs-start title = 'ベアラートークン cURL 例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` はテナント、認可ユーザー、付与されたスコープを返すため、接続テストに最適な呼び出しです。期限切れまたは失効したトークンでのリクエストは HTTP 401、トークンが保持していないスコープが必要なメソッドでのリクエストは HTTP 403 が返されます。

### Step 4 - Refreshing

[inline-code-attrs-start title = 'リフレッシュリクエスト cURL 例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'トークンリクエストボディ (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** オプション。元々付与されたスコープのサブセットに絞ります。 **/
    scope?: string
    resource?: string
}
[inline-code-end]

レスポンスはコード交換と同じ形です。リフレッシュトークンはローテーションします：各リフレッシュで新しい `refresh_token` が返され、30 秒の猶予ウィンドウ後に古いトークンが失効します。30 秒以上前にローテーションされたリフレッシュトークンを提示するとリプレイとみなされ、全体のグラントが失効します。FastComments が直接登録したパートナーアプリはローテーションの対象外で、同じリフレッシュトークンが返され、有効期限がさらに 30 日延長されます。

リフレッシュ時には、認可ユーザーが依然としてアカウントで API Admin 権限を保持しているか再確認されます。保持していない場合、グラントは失効し、レスポンスは `invalid_grant` になります。

### Revocation

[inline-code-attrs-start title = 'トークン失効リクエスト cURL 例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

リフレッシュトークンを失効させると、同じグラントから発行されたすべてのアクセストークンが失効します。アクセストークンを失効させると、そのトークンだけが失効します。エンドポイントはトークンが見つかったかどうかに関わらず、空の JSON オブジェクトで HTTP 200 を返します（RFC 7009 に準拠）。

ユーザーは FastComments ダッシュボードの **Connected Apps** から接続を失効させることもできます。そのアプリケーションのすべてのトークンは即座に使用できなくなります。