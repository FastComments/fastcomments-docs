FastCommentsは設定に環境変数を利用します。以下のリストは、On-Premに関連するサポート対象のすべての変数を示したものです。


| 変数                           | デフォルト                  | 説明                                                                                                                                               | 必須     | 例または有効な値                                       |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 環境の種類。                                                                                                                                  | はい      | production, dev                                       |
| MONGO_URI                      |                             | DB接続URI。                                                                                                                                 | はい      |                                                       |
| MONGO_ENABLE_SSL               | false                       | データベースへの接続にSSLを使用することを有効にします。                                                                                                      | いいえ       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Mongoへの接続時にCAに対して証明書を検証することを有効にします。                                                                               | いいえ       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pemファイル。                                                                                                                             | いいえ       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | システムに関する重要な通知の送信先メールアドレス。                                                                                      | いいえ       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | IPアドレスをハッシュ化するためのソルト。                                                                                                                     | はい      |                                                       |
| SESSION_SECRET                 |                             | セッションの署名に使用されるキー。                                                                                                                     | はい      |                                                       |
| SESSION_STORE_SECRET           |                             | ストレージ内のセッションの署名/ハッシュに使用されるキー。SESSION_SECRETとは異なる値である必要があります。                                                              | はい      |                                                       |
| HOSTNAME                       |                             | FastCommentsがデプロイされるホスト名(管理ダッシュボードなど)。ポートやプロトコルは含めないでください。                                            | はい      | example.com                                           |
| HOST_ADDR                      |                             | FastCommentsがデプロイされるアクセス可能なURI(管理ダッシュボードなど)。                                                                            | はい      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | メール設定(SMTP、ドメイン/プロバイダーのマッピングなど)が置かれているローカルファイルシステム上のパス。                                                   | はい      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | メールの「From Name」ヘッダー。                                                                                                                          | いいえ       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | メールフッターのロゴ。                                                                                                                                 | いいえ       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | EMAIL_CONFIG_PATH内の「defaultTransport」の上書き。同じ設定ファイルを異なる環境にデプロイする場合に便利です。                                     | いいえ       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | fastcomments.com上のアカウントのID。ライセンスキーの登録に使用されます。                                                                     | いいえ       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-Premライセンスキー。                                                                                                                            | いいえ       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy APIキー。指定しない場合は、gifピッカーを無効にする設定ルールを作成する必要があります。                                                     | いいえ       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | giphy統合に使用されます。ウィジェットのカスタマイズルールで上書きすることもできます。                                                                | いいえ       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | オプションのGPTベースのスパム検出など、openaiを利用した機能に使用されます。                                                                           | いいえ       |                                                       |
| CDN_HOST_ADDR                  |                             | アセットの取得元となるホスト名。デフォルトはHOSTNAMEの値です。                                                                     | いいえ       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 大きなファイル(エクスポートなど)の取得元となるホスト名。デフォルトはCDN_HOST_ADDRの値です。                                                | いいえ       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | エクスポートなどの大きなファイルの保存先。                                                                                                 | いいえ       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | メールの送信元となるホスト名。                                                                                                     | いいえ       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcommentsクッキーの名前。                                                                                                                         | いいえ       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | クッキーの「hostname」フィールドの値。先頭にドットを付けることを推奨します。                                                                          | いいえ       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | ユーザーのファイルアップロード、アバターなどに使用されます。未定義の場合はローカルFSがデフォルトになります。                                                                       | いいえ       |                                                       |
| S3_SECRET_KEY                  |                             | ユーザーのファイルアップロード、アバターなどに使用されます。                                                                                                          | いいえ       |                                                       |
| S3_REGION                      |                             | ユーザーのファイルアップロード、アバターなどに使用されます。                                                                                                          | いいえ       |                                                       |
| S3_BUCKET                      |                             | ユーザーのファイルアップロード、アバターなどに使用されます。                                                                                                          | いいえ       |                                                       |
| S3_HOST                        |                             | ユーザーのファイルアップロード、アバターなどに使用されます。                                                                                                          | いいえ       |                                                       |
| CACHE_DIR                      |                             | DBが利用できない場合に備えたオプションのオフラインキャッシュの保存場所。上位100件のコメントスレッドで定期的に更新されます。                       | いいえ       |                                                       |
| BACKUP_DIR                     |                             | DBが利用できない場合に備えたデータの保存場所。DBが利用できないときにコメントが送信されると、ここに保存され、後で処理されます。 | いいえ       |                                                       |

すべてのドメイン関連の変数は、`_HOST`または`_ADDR`の接尾辞を使用することに注意してください。その違いは以下の通りです:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH`には、以下の例のような形式のJSONファイルへのパスを指定する必要があります:

[inline-code-attrs-start title = 'メール設定の例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

上記の例では、`mailgun`というデフォルトの`SMTP`メールトランスポートを定義しています。また、`@yahoo.com`のメール専用に使用される特別なトランスポートも定義しています。配信を調整するために、特定のドメインに対して特定のプロバイダーや送信IPを使用することが望ましい場合があります。これはオプションです。

### DocumentDB

`DocumentDB`に接続する際は、デフォルト設定との互換性を保つために`MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`を指定する必要があります。