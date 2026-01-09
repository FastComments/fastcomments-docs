FastComments は設定に環境変数を使用します。以下の一覧はオンプレに関連するサポートされているすべての変数を概説しています。

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 環境の種類。                                                                                                                                       | はい     | production, dev                                       |
| MONGO_URI                      |                             | DB 接続 URI。                                                                                                                                     | はい     |                                                       |
| MONGO_ENABLE_SSL               | false                       | データベースへの接続に SSL を使用するかを有効にします。                                                                                             | いいえ   | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Mongo に接続する際に CA に対して証明書を検証するかを有効にします。                                                                                      | いいえ   | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem ファイル。                                                                                                                         | いいえ   | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | 重要なシステム関連の通知を送るメールアドレス。                                                                                                        | いいえ   | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | IP アドレスをハッシュ化するためのソルト。                                                                                                           | はい     |                                                       |
| SESSION_SECRET                 |                             | セッションの署名に使用されるキー。                                                                                                                 | はい     |                                                       |
| SESSION_STORE_SECRET           |                             | ストレージ内のセッションを署名/ハッシュするために使用されるキー。SESSION_SECRET とは異なる必要があります。                                             | はい     |                                                       |
| HOSTNAME                       |                             | FastComments がデプロイされているホスト名（管理ダッシュボード等）。ポートやプロトコルを含めないでください。                                               | はい     | example.com                                           |
| HOST_ADDR                      |                             | FastComments がデプロイされているアクセス可能な URI（管理ダッシュボード等）。                                                                       | はい     | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | メール構成（SMTP、ドメイン/プロバイダのマッピング等）があるローカルファイルシステム上のパス。                                                          | はい     | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | メールの "From Name" ヘッダー。                                                                                                                     | いいえ   | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | メールフッターロゴ。                                                                                                                               | いいえ   | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | EMAIL_CONFIG_PATH の "defaultTransport" を上書きします。同じ設定ファイルを異なる環境にデプロイする際に便利です。                                           | いいえ   | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | fastcomments.com 上のアカウント ID。ライセンスキーを登録するために使用されます。                                                                        | いいえ   |                                                       |
| ON_PREM_LICENSE_KEY            |                             | オンプレ用ライセンスキー。                                                                                                                           | いいえ   |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API キー。指定されていない場合は、GIF ピッカーを無効にする設定ルールを作成する必要があります。                                                     | いいえ   |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Giphy 統合で使用されます。ウィジェットのカスタマイズルールで上書きすることもできます。                                                                   | いいえ   | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | オプションの GPT ベースのスパム検出など、OpenAI を使用する機能に使用されます。                                                                          | いいえ   |                                                       |
| CDN_HOST_ADDR                  |                             | アセットが取得されるホスト名。デフォルトは HOSTNAME の値になります。                                                                                    | いいえ   | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 大きなファイル（エクスポートなど）が取得されるホスト名。デフォルトは CDN_HOST_ADDR の値になります。                                                       | いいえ   | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | エクスポートなどの大きなファイルをどこに保存するか。                                                                                                   | いいえ   | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | メールが送信されるホスト名。                                                                                                                         | いいえ   | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments クッキーの名前。                                                                                                                       | いいえ   |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | クッキーの "hostname" フィールドの値。ドットをプレフィックスすることを推奨します。                                                                       | いいえ   | .example.com                                          |
| S3_ACCESS_KEY                  |                             | ユーザーファイルのアップロード、アバター等に使用。未定義の場合はローカル FS がデフォルトになります。                                                      | いいえ   |                                                       |
| S3_SECRET_KEY                  |                             | ユーザーファイルのアップロード、アバター等に使用。                                                                                                     | いいえ   |                                                       |
| S3_REGION                      |                             | ユーザーファイルのアップロード、アバター等に使用。                                                                                                     | いいえ   |                                                       |
| S3_BUCKET                      |                             | ユーザーファイルのアップロード、アバター等に使用。                                                                                                     | いいえ   |                                                       |
| S3_HOST                        |                             | ユーザーファイルのアップロード、アバター等に使用。                                                                                                     | いいえ   |                                                       |
| CACHE_DIR                      |                             | DB が利用できない場合のオプションのオフラインキャッシュを保存する場所。定期的にトップ100のコメントスレッドで更新されます。                                   | いいえ   |                                                       |
| BACKUP_DIR                     |                             | DB が利用できない場合にデータを保存する場所。DB が利用できないときにコメントが送信されるとここに保存され、後で処理されます。                                 | いいえ   |                                                       |

ドメイン関連のすべての変数は `_HOST` または `_ADDR` の後置を使用することに注意してください。違いは次のとおりです：

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

The `EMAIL_CONFIG_PATH` should contain a path to a JSON file with the following example format:

[inline-code-attrs-start title = 'メール設定'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "defaultDKIM": {
        "domainName": "mycompany.org",
        "keySelector": "2024",
        "privateKey": "-----BEGIN PRIVATE KEY-----\nABCDEFG\n-----END PRIVATE KEY-----"
    },
    "providerTransports": {
        "yahoo.com": "specialTransport"
    },
    "defaultTransport": "mailgun",
    "transports": {
        "mailgun": {
            "host": "smtp.mailgun.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@somewhere.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        },
        "specialTransport": {
            "host": "smtp.someplace.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@example.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        }
    }
}
[inline-code-end]

上記の例では、`mailgun` という名前のデフォルトの `SMTP` メールトランスポートを定義しています。さらに、`@yahoo.com` メール用に特定に使用する特別なトランスポートも定義しています。特定のドメインに対して配信を最適化するために特定のプロバイダや送信 IP を使用することが望ましい場合があります。これは任意です。

### DocumentDB

`DocumentDB` に接続する場合、デフォルト設定と互換性を持たせるために `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` を指定することを推奨します。