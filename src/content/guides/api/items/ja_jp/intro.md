### FastComments API

FastComments は多数のリソースとやり取りするための API を提供します。プラットフォームとの統合を構築したり、独自のクライアントを作成したりできます！

このドキュメントでは、API がサポートするすべてのリソースと、そのリクエストおよびレスポンスタイプが記載されています。

エンタープライズ顧客向けには、すべての API アクセスが監査ログに記録されます。

### 生成された SDK

FastComments は現在、コードから [API Spec](https://fastcomments.com/js/swagger.json) を生成しています（まだ完全ではありませんが、多くの API が含まれています）。

また、人気のある言語向けの SDK も用意しています：

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

これらのルートは **サーバー** から呼び出すことを想定しています。__絶対に__ ブラウザから呼び出さないでください。そうすると API キーが露出し、ページのソースコードを閲覧できる人があなたのアカウントにフルアクセスできてしまいます！

#### 認証オプション 1 - ヘッダー

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### 認証オプション 2 - クエリパラメータ

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### 認証オプション 3 - OAuth ベアラートークン

- Header: `Authorization: Bearer fcat_...`

Zapier などのサードパーティアプリケーションや [MCP サーバー](https://docs.fastcomments.com/guide-llm-kit.html) のクライアントは、API キーの代わりに OAuth を通じてトークンを取得します。そのトークンはここにあるすべてのエンドポイントで使用できます。テナントはトークンに含まれるため、`tenantId` は任意ですが、指定する場合はトークンと一致する必要があります。`GET` リクエストには `read` スコープが必要で、その他のメソッドには `write` スコープが必要です。クライアント登録、PKCE、リフレッシュ、トークン失効を含む全フローは [OAuth Authorization](#oauth) に記載されています。ディスカバリーは `https://fastcomments.com/.well-known/oauth-authorization-server` から開始します。

### 自分の書き込みの読み取り

FastComments はアクティブ-アクティブの可用性を提供します。データセンターからのリクエストは、[最も近いプレゼンスポイント](https://sophon.fastcomments.com/) にルーティングされます。これは自動的に行われ、通常は「書いたものをすぐに読む」セマンティクスが観測できます。自分の書き込みを確実に読む必要がある場合は、対象のリージョンを API ホストとして使用し、リクエストをそのリージョンに固定できます（ただし、ほとんどの統合では通常必要ありません）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

ただし、これを行う場合はフォールバックを定義した方がよいでしょう。過去にエントリーポイントノードを廃止し、切り替え時に新しい名前を使用しているためです。