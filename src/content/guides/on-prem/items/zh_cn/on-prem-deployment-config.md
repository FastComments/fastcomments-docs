FastComments 使用环境变量进行配置。以下列表概述了与 On-Prem 相关的所有受支持变量。

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 环境类型。                                                                                                                                         | Yes      | production, dev                                       |
| MONGO_URI                      |                             | 数据库连接 URI。                                                                                                                                    | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | 启用使用 SSL 连接到数据库。                                                                                                                         | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | 启用在连接到 Mongo 时根据 CA 验证证书。                                                                                                             | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem 文件。                                                                                                                             | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | 接收重要系统相关通知的电子邮件地址。                                                                                                                 | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | 用于哈希化 IP 地址的盐值。                                                                                                                         | Yes      |                                                       |
| SESSION_SECRET                 |                             | 用于签名会话的密钥。                                                                                                                                | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | 用于在存储中签名/哈希会话的密钥。必须与 SESSION_SECRET 不同。                                                                                       | Yes      |                                                       |
| HOSTNAME                       |                             | 部署 FastComments（管理面板等）所在的主机名。不应包含端口或协议。                                                                                     | Yes      | example.com                                           |
| HOST_ADDR                      |                             | 可访问的 URI，FastComments（管理面板等）部署所在位置。                                                                                               | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | 本地文件系统上存放电子邮件配置（SMTP、域/提供商映射等）的路径。                                                                                        | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | 电子邮件 “From Name” 头。                                                                                                                           | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | 电子邮件页脚徽标。                                                                                                                                  | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | 覆盖 EMAIL_CONFIG_PATH 中的 "defaultTransport"。在将相同配置文件部署到不同环境时很有用。                                                               | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | 您在 fastcomments.com 上帐户的 ID。用于注册您的许可证密钥。                                                                                           | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | 本地部署的许可证密钥。                                                                                                                              | No       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API 密钥。如果未指定，您应创建一条配置规则以禁用 gif 选择器。                                                                                  | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | 用于 giphy 集成。也可以通过小部件自定义规则覆盖。                                                                                                     | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | 用于 OpenAI 驱动的功能，例如可选的基于 GPT 的垃圾评论检测。                                                                                          | No       |                                                       |
| CDN_HOST_ADDR                  |                             | 获取资源的主机名。默认为 HOSTNAME 的值。                                                                                                            | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 获取大文件（如导出）的主机名。默认为 CDN_HOST_ADDR 的值。                                                                                            | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | 存储大文件（如导出）的方式。                                                                                                                         | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | 发送电子邮件时应使用的主机名。                                                                                                                       | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments cookie 的名称。                                                                                                                         | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | cookie “hostname” 字段的值。建议以点号作为前缀。                                                                                                      | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | 用于用户文件上传、头像等。如果未定义则默认为本地文件系统。                                                                                             | No       |                                                       |
| S3_SECRET_KEY                  |                             | 用于用户文件上传、头像等。                                                                                                                           | No       |                                                       |
| S3_REGION                      |                             | 用于用户文件上传、头像等。                                                                                                                           | No       |                                                       |
| S3_BUCKET                      |                             | 用于用户文件上传、头像等。                                                                                                                           | No       |                                                       |
| S3_HOST                        |                             | 用于用户文件上传、头像等。                                                                                                                           | No       |                                                       |
| CACHE_DIR                      |                             | 存放可选离线缓存的位置，以在数据库不可用时使用。会定期刷新，保存排名前 100 的评论线程。                                                                   | No       |                                                       |
| BACKUP_DIR                     |                             | 存放在数据库不可用时的数据的位置。如果在数据库不可用时提交评论，它将存放到这里，并在稍后处理。                                                             | No       |                                                       |

注意，所有与域相关的变量使用 `_HOST` 或 `_ADDR` 后缀。区别是：

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` 应包含一个指向 JSON 文件的路径，该文件具有以下示例格式：

[inline-code-attrs-start title = '电子邮件配置'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

在上述示例中，我们定义了一个名为 `mailgun` 的默认 `SMTP` 邮件传输。我们还为 `@yahoo.com` 的电子邮件专门定义了一个特殊传输。在某些场景中，希望针对某个域使用特定提供商或发送 IP 以优化投递。这是可选的。

### DocumentDB

当连接到 `DocumentDB` 时，您需要指定 `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` 以兼容默认设置。