### FastComments API

FastComments は多くのリソースとやり取りするための API を提供します。プラットフォームとの統合を構築したり、独自のクライアントを作成したりできます！

このドキュメントでは、API がサポートするすべてのリソースと、そのリクエストおよびレスポンスタイプが記載されています。

エンタープライズ顧客向けには、すべての API アクセスが監査ログに記録されます。

### 生成された SDK

FastComments は現在、コードから [API Spec](https://fastcomments.com/js/swagger.json) を生成しています（まだ完全ではありませんが、多くの API が含まれています）。

また、人気のある言語向けに SDK も提供しています：

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

API は、[API キー](https://fastcomments.com/auth/my-account/api-secret) を `X-API-KEY` ヘッダーまたは `API_KEY` クエリパラメータのいずれかで渡すことで認証されます。API 呼び出しを行うには `tenantId` も必要です。`tenantId` は API キーと同じページから取得できます。

### セキュリティに関する注意

これらのルートは **サーバー** から呼び出すことを想定しています。__絶対に__ ブラウザーから呼び出さないでください。そうすると API キーが露出し、ページのソースコードを閲覧できるすべての人があなたのアカウントにフルアクセスできるようになります！

#### 認証オプション 1 - ヘッダー

- ヘッダー: `X-API-KEY`
- ヘッダー: `X-TENANT-ID`

#### 認証オプション 2 - クエリパラメータ

- クエリパラメータ: `API_KEY`
- クエリパラメータ: `tenantId`

#### 認証オプション 3 - OAuth ベアラートークン

- ヘッダー: `Authorization: Bearer fcat_...`

[MCP サーバー](https://docs.fastcomments.com/guide-llm-kit.html) を介して接続するアプリケーションは、API キーの代わりに OAuth でトークンを取得します。そのトークンはここにあるすべてのエンドポイントで機能します。テナントはトークンに含まれるため、`tenantId` はオプションですが、指定する場合はトークンと一致である必要があります。`GET` リクエストには `read` スコープが必要で、その他のメソッドには `write` スコープが必要です。ディスカバリは `https://fastcomments.com/.well-known/oauth-authorization-server` から開始されます。

### 自分の書き込みの読み取り

FastComments はアクティブ-アクティブの可用性を提供します。データセンターからのリクエストは、[最も近いプレゼンスポイント](https://sophon.fastcomments.com/) にルーティングされます。これは自動的に行われ、通常は「書いたものをすぐに読む」セマンティクスが観測できます。自分の書き込みを確実に読む必要がある場合は、特定のリージョンを API ホストとして使用してリクエストを固定できます（ただし、ほとんどの統合では通常必要ありません）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

このように設定する場合、過去にエントリーポイントノードが廃止され、スイッチオーバー用に新しい名前が使用されているため、フォールバックを定義した方がよいことに注意してください。

---