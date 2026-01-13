FastComments 使用環境變數來進行設定。以下列出所有與 On-Prem 相關且支援的變數。

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 環境類型。                                                                                                                                         | Yes      | production, dev                                       |
| MONGO_URI                      |                             | 資料庫連線 URI。                                                                                                                                    | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | 啟用使用 SSL 來連線資料庫。                                                                                                                         | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | 在連線到 Mongo 時啟用對憑證與 CA 的驗證。                                                                                                          | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem 檔案。                                                                                                                            | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | 接收重要系統相關通知的電子郵件地址。                                                                                                               | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | 用於對 IP 位址進行雜湊的 Salt。                                                                                                                     | Yes      |                                                       |
| SESSION_SECRET                 |                             | 用於簽署會話的金鑰。                                                                                                                                | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | 用於在儲存中簽署/雜湊會話的金鑰。必須不同於 SESSION_SECRET。                                                                                         | Yes      |                                                       |
| HOSTNAME                       |                             | 部署 FastComments（管理面板等）的主機名稱。不得包含連接埠或協定。                                                                                    | Yes      | example.com                                           |
| HOST_ADDR                      |                             | 部署 FastComments（管理面板等）的可存取 URI。                                                                                                       | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | 本機檔案系統上放置電子郵件設定（SMTP、網域／提供者對應等）的路徑。                                                                                     | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | 電子郵件 "From Name" 標頭。                                                                                                                         | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | 電子郵件頁尾 logo。                                                                                                                                | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | 覆寫 EMAIL_CONFIG_PATH 中的 "defaultTransport"。在將相同設定檔部署到不同環境時很有用。                                                                  | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | 您在 fastcomments.com 帳戶的 ID。用於註冊授權金鑰。                                                                                                   | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | 一組 On-Prem 授權金鑰。                                                                                                                             | No       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API Key。如果未指定，應建立一個設定規則以停用 gif 選取器。                                                                                      | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | 用於 giphy 的整合。也可透過元件自訂規則覆寫。                                                                                                        | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | 用於 OpenAI 驅動的功能，例如可選的基於 GPT 的垃圾郵件檢測。                                                                                           | No       |                                                       |
| CDN_HOST_ADDR                  |                             | 資源要從何處擷取的主機名稱。預設為 HOSTNAME 的值。                                                                                                   | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 大型檔案（例如匯出）要從何處擷取的主機名稱。預設為 CDN_HOST_ADDR 的值。                                                                                 | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | 大型檔案（如匯出檔）應儲存的位置。                                                                                                                   | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | 電子郵件應由哪個主機名稱發送。                                                                                                                       | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments Cookie 的名稱。                                                                                                                        | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Cookie 的 "hostname" 欄位值。建議前面加上點（dot）。                                                                                                | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | 用於使用者上傳檔案、頭像等。如果未定義則預設為本機檔案系統。                                                                                          | No       |                                                       |
| S3_SECRET_KEY                  |                             | 用於使用者上傳檔案、頭像等。                                                                                                                         | No       |                                                       |
| S3_REGION                      |                             | 用於使用者上傳檔案、頭像等。                                                                                                                         | No       |                                                       |
| S3_BUCKET                      |                             | 用於使用者上傳檔案、頭像等。                                                                                                                         | No       |                                                       |
| S3_HOST                        |                             | 用於使用者上傳檔案、頭像等。                                                                                                                         | No       |                                                       |
| CACHE_DIR                      |                             | 用於儲存選用的離線快取位置，以便在資料庫不可用時使用。會定期以前 100 個熱門留言串更新。                                                                 | No       |                                                       |
| BACKUP_DIR                     |                             | 用於在資料庫不可用時儲存資料的位置。如果在資料庫不可用時有留言提交，會先放在此處，稍後再處理。                                                           | No       |                                                       |

請注意，所有與網域相關的變數都使用 `_HOST` 或 `_ADDR` 後綴。兩者的差別為：

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

The `EMAIL_CONFIG_PATH` should contain a path to a JSON file with the following example format:

[inline-code-attrs-start title = '電子郵件設定'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

在上方範例中，我們定義了一個名為 `mailgun` 的預設 `SMTP` 電子郵件傳輸。我們也定義了一個專門用於 `@yahoo.com` 電子郵件的特殊傳輸。在某些情境下，針對特定網域使用特定提供者或發送 IP 以調整投遞是可取的。此項為選用。

### DocumentDB

連線到 `DocumentDB` 時，您會想要指定 `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`，以相容於預設設定。

---