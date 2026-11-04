FastComments 使用環境變數進行配置。以下列表列出所有與 On-Prem 相關的支援變數。


| 變數                           | 預設值                      | 說明                                                                                                                                               | 必填     | 範例或有效值                                          |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 環境類型。                                                                                                                                          | 是       | production, dev                                       |
| MONGO_URI                      |                             | 資料庫連線 URI。                                                                                                                                    | 是       |                                                       |
| MONGO_ENABLE_SSL               | false                       | 啟用使用 SSL 連線到資料庫。                                                                                                                          | 否       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | 連線到 Mongo 時啟用對 CA 驗證憑證。                                                                                                                  | 否       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem 檔案。                                                                                                                             | 否       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | 重要系統相關通知應發送的電子郵件地址。                                                                                                                | 否       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | 用於雜湊 IP 位址的鹽值。                                                                                                                            | 是       |                                                       |
| SESSION_SECRET                 |                             | 用於簽署 session 的金鑰。                                                                                                                           | 是       |                                                       |
| SESSION_STORE_SECRET           |                             | 用於簽署/雜湊儲存中 session 的金鑰。必須與 SESSION_SECRET 不同。                                                                                     | 是       |                                                       |
| HOSTNAME                       |                             | FastComments 部署的主機名稱（管理後台等）。不應包含連接埠或通訊協定。                                                                                | 是       | example.com                                           |
| HOST_ADDR                      |                             | FastComments 部署的可存取 URI（管理後台等）。                                                                                                       | 是       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | 本機檔案系統上電子郵件設定（SMTP、網域/供應商對應等）所在的路徑。                                                                                     | 是       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | 電子郵件「寄件者名稱」標頭。                                                                                                                        | 否       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | 電子郵件頁尾標誌。                                                                                                                                  | 否       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | 覆寫 EMAIL_CONFIG_PATH 中的「defaultTransport」。適用於將相同設定檔部署到不同環境。                                                                  | 否       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | 您在 fastcomments.com 上的帳號 ID。用於註冊您的授權金鑰。                                                                                            | 否       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-Prem 授權金鑰。                                                                                                                                  | 否       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API 金鑰。若未指定，您應建立一條停用 gif 選擇器的設定規則。                                                                                     | 否       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | 用於 giphy 整合。也可以透過小工具自訂規則覆寫。                                                                                                      | 否       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | 用於 openai 驅動的功能，例如可選的基於 GPT 的垃圾訊息偵測。                                                                                          | 否       |                                                       |
| CDN_HOST_ADDR                  |                             | 取得資源檔的主機名稱。預設為 HOSTNAME 的值。                                                                                                        | 否       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 取得大型檔案（如匯出檔）的主機名稱。預設為 CDN_HOST_ADDR 的值。                                                                                      | 否       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | 大型檔案（如匯出檔）應儲存的位置。                                                                                                                  | 否       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | 電子郵件應從哪個主機名稱發送。                                                                                                                      | 否       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments cookie 的名稱。                                                                                                                        | 否       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | cookie「hostname」欄位的值。建議以點號作為前綴。                                                                                                    | 否       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | 用於使用者檔案上傳、頭像等。若未定義則預設使用本機檔案系統。                                                                                          | 否       |                                                       |
| S3_SECRET_KEY                  |                             | 用於使用者檔案上傳、頭像等。                                                                                                                        | 否       |                                                       |
| S3_REGION                      |                             | 用於使用者檔案上傳、頭像等。                                                                                                                        | 否       |                                                       |
| S3_BUCKET                      |                             | 用於使用者檔案上傳、頭像等。                                                                                                                        | 否       |                                                       |
| S3_HOST                        |                             | 用於使用者檔案上傳、頭像等。                                                                                                                        | 否       |                                                       |
| CACHE_DIR                      |                             | 儲存選用的離線快取的位置，用於資料庫無法使用時。會定期以前 100 個留言串重新整理。                                                                    | 否       |                                                       |
| BACKUP_DIR                     |                             | 資料庫無法使用時儲存資料的位置。若在資料庫無法使用時提交留言，留言會存放在這裡並於稍後處理。                                                          | 否       |                                                       |

請注意，所有與網域相關的變數都使用 `_HOST` 或 `_ADDR` 後綴。兩者的差異如下：

- `_HOST`：`example.com`
- `_ADDR`：`https://example.com`

`EMAIL_CONFIG_PATH` 應包含指向具有以下範例格式的 JSON 檔案的路徑：

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

在上面的範例中，我們定義了一個名為 `mailgun` 的預設 `SMTP` 電子郵件傳輸方式。我們還定義了一個專門用於
`@yahoo.com` 電子郵件的特殊傳輸方式。在某些情況下，為特定網域使用特定的供應商或發送 IP 以調整送達率是理想的作法。這是選用的。

### DocumentDB

連線到 `DocumentDB` 時，您需要指定 `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` 以相容於預設設定。