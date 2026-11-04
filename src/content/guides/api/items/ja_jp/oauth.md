FastCommentsはOAuth 2.1認可サーバーです。アプリケーションは1つのFastCommentsアカウントに紐付けられたトークンを取得し、このガイドのすべてのエンドポイントでAPIキーの代わりに使用できます。Zapierアプリ、MCPサーバー、およびその他のサードパーティ統合は、この方法で接続します。

トークンはPKCEを用いた認可コードフローを通じて発行されます。クライアントクレデンシャルやインプリシットグラントはありません。

### ディスカバリー

エンドポイントの場所、サポートされているグラント、および認証方法は、標準のメタデータURLで公開されています:

[inline-code-attrs-start title = '認可サーバーメタデータ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

記載されているエンドポイント:

[inline-code-attrs-start title = 'OAuthエンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EUリージョンのアカウントは、同じパスで `https://eu.fastcomments.com` をissuerとして使用します。

### クライアントの登録

クライアントは、フローを開始する前に `client_id` と登録済みの `redirect_uri` が必要です。取得するには2つの方法があります:

- **動的クライアント登録。** RFC 7591に従い、JSONボディ(`redirect_uris`、`client_name`、`client_uri`、`logo_uri`、`token_endpoint_auth_method`)を付けて `POST /oauth/register` を送信します。レスポンスには `client_id` が含まれ、機密クライアントの場合は `client_secret` も含まれます。登録は認証なしで行われ、IPごとにレート制限されます。
- **クライアントIDメタデータドキュメント。** クライアントは、自身が管理する `https` URLを `client_id` として使用します。FastCommentsはそのURLを取得し、同じメタデータフィールドを読み取ります。登録呼び出しは不要です。

Zapierなど、FastCommentsダッシュボードに表示されるパートナーアプリケーションは、FastCommentsによって直接登録されます。マーケットプレイスへの掲載を構築していて、ファーストパーティクライアントが必要な場合は、サポートにお問い合わせください。

### スコープ

[inline-code-attrs-start title = 'スコープ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

スコープを要求しないリクエストには両方が付与されます。ユーザーには、同意ページで要求されたスコープが表示されます。これら2つ以外のスコープを要求すると、`invalid_scope` で失敗します。

### ステップ1 - 認可リクエスト

ユーザーのブラウザを認可エンドポイントに誘導します。`S256` メソッドによるPKCEは、すべてのクライアントに必須です。

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
    /** Must exactly match one of the client's registered redirect URIs. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    resource?: string
}
[inline-code-end]

必要に応じてユーザーがFastCommentsにサインインすると、アプリケーション名、接続先のアカウント、要求されたスコープを示す同意ページが表示されます。ユーザーはそのアカウントで**API Admin**権限を持っている必要があります。それ以外のユーザーには、同意フォームの代わりに権限エラーが表示されます。承認すると、`code` と `state` 付きでブラウザが `redirect_uri` にリダイレクトされます。拒否すると、`error=access_denied` 付きでリダイレクトされます。

認可コードは10分間有効で、1回のみ交換できます。同じコードを2回目に交換すると、最初の交換で発行されたすべてのトークンが失効します。

### ステップ2 - トークンリクエスト

コードをトークンと交換します。ボディはフォームエンコードです。機密クライアントは `client_secret_basic`(HTTP Basic)または `client_secret_post`(ボディにシークレットを含める)で認証します。パブリッククライアントは `client_id` のみを送信します。

[inline-code-attrs-start title = 'トークンリクエストcURL例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'トークンリクエストボディ(authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Confidential clients only. May be sent as HTTP Basic auth instead. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'トークンレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    scope: string
}
[inline-code-end]

エラーはRFC 6749に従います: `error` と `error_description` を含むJSONボディ、`invalid_request`、`invalid_grant`、`invalid_scope`、`invalid_target`、`unsupported_grant_type` はHTTP 400、`invalid_client` はHTTP 401、レート制限時はHTTP 429。

### ステップ3 - APIの呼び出し

アクセストークンをベアラートークンとして送信します。テナントはトークンから推測されるため、`tenantId` は省略可能です。指定する場合はトークンと一致している必要があり、一致しないとリクエストは失敗します。

[inline-code-attrs-start title = 'ベアラートークンcURL例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` はテナント、認可ユーザー、付与されたスコープを返すため、接続テストに適した呼び出しです。期限切れまたは失効したトークンでのリクエストはHTTP 401を返します。トークンが保持していないスコープを必要とするメソッドのリクエストはHTTP 403を返します。

### ステップ4 - 更新

[inline-code-attrs-start title = '更新リクエストcURL例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'トークンリクエストボディ(refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Narrows to a subset of the scopes originally granted. **/
    scope?: string
    resource?: string
}
[inline-code-end]

レスポンスはコード交換と同じ形式です。リフレッシュトークンはローテーションします: 各更新は新しい `refresh_token` を返し、同時リクエスト用の30秒の猶予期間の後、古いトークンを失効させます。30秒以上前にローテーションされたリフレッシュトークンの提示はリプレイとみなされ、グラント全体が失効します。FastCommentsによって登録されたパートナーアプリケーションはローテーションが免除され、有効期限がさらに30日延長された同じリフレッシュトークンが返されます。

更新時には、認可ユーザーが引き続きアカウントのAPI Admin権限を持っているかも再確認されます。持っていない場合、グラントは失効し、レスポンスは `invalid_grant` になります。

### 失効

[inline-code-attrs-start title = '失効リクエストcURL例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

リフレッシュトークンを失効させると、同じグラントから発行されたすべてのアクセストークンが失効します。アクセストークンを失効させると、そのトークンのみが失効します。RFC 7009に従い、このエンドポイントはトークンが見つかったかどうかに関係なく、空のJSONオブジェクトと共にHTTP 200を返します。

ユーザーは、FastCommentsダッシュボードの**接続済みアプリ**から接続を失効させることもできます。そのアプリケーションのすべてのトークンは直ちに機能しなくなります。