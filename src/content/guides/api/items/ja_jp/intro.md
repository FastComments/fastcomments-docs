### FastComments の API

FastComments は多くのリソースとやり取りするための API を提供します。プラットフォームとの統合を構築したり、独自のクライアントを構築したりできます。

このドキュメントでは、API がサポートするすべてのリソースを、リクエストおよびレスポンスタイプとともに記載しています。

エンタープライズのお客様向けに、すべての API アクセスは監査ログに記録されます。

### Generated SDKs

FastComments は現在、コードから [API 仕様](https://fastcomments.com/js/swagger.json) を生成しています（まだ完全ではありませんが、多くの API を含んでいます）。

また、人気のある言語向けの SDK も用意しています:

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

API は、[API キー](https://fastcomments.com/auth/my-account/api-secret) を `X-API-KEY` ヘッダーまたは `API_KEY` クエリパラメータとして渡すことで認証されます。API コールを行うには `tenantId` も必要です。これは API キーと同じページから取得できます。

### セキュリティに関する注意

これらのルートは **サーバー** から呼び出すことを想定しています。__ブラウザから絶対に呼び出さないでください__。ブラウザから呼び出すと API キーが露出し、ページのソースコードを表示できる人にアカウントへの完全なアクセス権が与えられてしまいます。

#### 認証オプション 1 - ヘッダー

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### 認証オプション 2 - クエリパラメータ

- Query Param: `API_KEY`
- Query Param: `tenantId`

### 自分の書き込みの読み取り

FastComments はアクティブ・アクティブ構成による可用性を提供します。お客様のデータセンターからのリクエストは、お客様に最も近い [最寄りのプレゼンス・ポイント](https://sophon.fastcomments.com/) にルーティングされます。これは自動で行われ、通常は書き込み後に自分の書き込みを読み取れる整合性（read-your-write）が観察できます。自分の書き込みを確実に読みたい場合は、そのリージョンを API ホストとして使用してリクエストを特定のリージョンにピン留めできます（ただし、ほとんどの統合では通常必要ありません）:

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

これを行う場合、過去にエントリポイントノードを非推奨にして切り替え時に新しい名前を使用したことがあるため、フォールバックを定義しておくことをお勧めします。

---