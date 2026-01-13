### FastComments の API

FastComments は多数のリソースとやり取りするための API を提供しています。プラットフォームと統合を構築したり、独自のクライアントを作成したりできます！

このドキュメントでは、API がサポートするすべてのリソースを、リクエストとレスポンスの型とともに記載しています。

エンタープライズ顧客向けに、すべての API アクセスは監査ログに記録されます。

### 生成された SDK

FastComments はコードから [API 仕様](https://fastcomments.com/js/swagger.json) を生成するようになりました（まだ完全ではありませんが、多くの API が含まれています）。

We also now have SDKs for popular languages:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### 認証

API は、[API キー](https://fastcomments.com/auth/my-account/api-secret) を `X-API-KEY` ヘッダーまたは `API_KEY` クエリパラメータのいずれかとして渡すことで認証されます。API 呼び出しには `tenantId` も必要です。これは API キー と同じページから取得できます。

### セキュリティに関する注意

これらのルートは**サーバー**から呼び出すことを想定しています。__ブラウザから呼び出さないでください__。そうすると API キーが公開されてしまい、ページのソースコードを閲覧できる誰にでもアカウントへの完全なアクセス権が与えられてしまいます！

#### 認証オプション 1 - ヘッダー

- ヘッダー: `X-API-KEY`
- ヘッダー: `X-TENANT-ID`

#### 認証オプション 2 - クエリパラメータ

- クエリパラメータ: `API_KEY`
- クエリパラメータ: `tenantId`

---