FastComments는 구성에 환경 변수를 사용합니다. 다음 목록은 온프렘(On-Prem)에 관련된 모든 지원 변수를 설명합니다.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 환경 타입.                                                                                                                                         | 예       | production, dev                                       |
| MONGO_URI                      |                             | DB 연결 URI.                                                                                                                                        | 예       |                                                       |
| MONGO_ENABLE_SSL               | false                       | 데이터베이스에 연결할 때 SSL 사용을 활성화합니다.                                                                                                   | 아니오   | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Mongo에 연결할 때 CA에 대해 인증서를 검증하도록 활성화합니다.                                                                                       | 아니오   | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem 파일.                                                                                                                              | 아니오   | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | 중요한 시스템 관련 알림을 받을 이메일 주소.                                                                                                        | 아니오   | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | IP 주소 해시를 위한 솔트.                                                                                                                           | 예       |                                                       |
| SESSION_SECRET                 |                             | 세션 서명에 사용되는 키.                                                                                                                            | 예       |                                                       |
| SESSION_STORE_SECRET           |                             | 스토리지에 있는 세션을 서명/해시하는 데 사용되는 키. SESSION_SECRET와 달라야 합니다.                                                                 | 예       |                                                       |
| HOSTNAME                       |                             | FastComments가 배포된 호스트명(관리 대시보드 등). 포트나 프로토콜을 포함하면 안 됩니다.                                                              | 예       | example.com                                           |
| HOST_ADDR                      |                             | FastComments가 배포된 접근 가능한 URI(관리 대시보드 등).                                                                                             | 예       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | 이메일 구성(SMTP, 도메인/제공자 매핑 등)이 위치한 로컬 파일 시스템의 경로.                                                                         | 예       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | 이메일 "From Name" 헤더.                                                                                                                            | 아니오   | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | 이메일 바닥글 로고.                                                                                                                                 | 아니오   | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | EMAIL_CONFIG_PATH의 "defaultTransport"에 대한 오버라이드. 동일한 구성 파일을 다른 환경에 배포할 때 유용합니다.                                      | 아니오   | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | fastcomments.com의 계정 ID. 라이선스 키를 등록하는 데 사용됩니다.                                                                                    | 아니오   |                                                       |
| ON_PREM_LICENSE_KEY            |                             | 온프렘 라이선스 키.                                                                                                                                  | 아니오   |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API 키. 지정하지 않으면 gif 선택기를 비활성화하는 구성 규칙을 만들어야 합니다.                                                                 | 아니오   |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | giphy 통합에 사용됩니다. 위젯 커스터마이제이션 규칙으로도 재정의할 수 있습니다.                                                                     | 아니오   | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | 선택적 GPT 기반 스팸 감지 같은 OpenAI 기반 기능에 사용됩니다.                                                                                        | 아니오   |                                                       |
| CDN_HOST_ADDR                  |                             | 에셋을 가져올 호스트명. 기본값은 HOSTNAME의 값입니다.                                                                                               | 아니오   | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 대용량 파일(내보내기 등)을 가져올 호스트명. 기본값은 CDN_HOST_ADDR의 값입니다.                                                                      | 아니오   | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | 내보내기와 같은 대용량 파일을 저장할 위치.                                                                                                          | 아니오   | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | 이메일이 발송되는 호스트명.                                                                                                                         | 아니오   | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments 쿠키의 이름.                                                                                                                           | 아니오   |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | 쿠키의 "hostname" 필드 값. 접두사로 점을 권장합니다.                                                                                                 | 아니오   | .example.com                                          |
| S3_ACCESS_KEY                  |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다. 정의되지 않으면 로컬 파일 시스템이 기본입니다.                                                            | 아니오   |                                                       |
| S3_SECRET_KEY                  |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | 아니오   |                                                       |
| S3_REGION                      |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | 아니오   |                                                       |
| S3_BUCKET                      |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | 아니오   |                                                       |
| S3_HOST                        |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | 아니오   |                                                       |
| CACHE_DIR                      |                             | DB를 사용할 수 없을 때의 선택적 오프라인 캐시를 저장할 위치. 상위 100개 댓글 스레드로 주기적으로 갱신됩니다.                                           | 아니오   |                                                       |
| BACKUP_DIR                     |                             | DB를 사용할 수 없을 때 데이터를 저장할 위치. DB가 사용 불가능할 때 댓글이 제출되면 여기에 저장되며 나중에 처리됩니다.                                 | 아니오   |                                                       |

모든 도메인 관련 변수는 `_HOST` 또는 `_ADDR` 접미사를 사용한다는 점에 유의하세요. 차이점은:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

The `EMAIL_CONFIG_PATH` should contain a path to a JSON file with the following example format:

[inline-code-attrs-start title = '이메일 구성'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

위 예제에서는 `mailgun`이라는 기본 `SMTP` 이메일 트랜스포트를 정의합니다. 또한 `@yahoo.com` 이메일에 대해 특별히 사용하는 전용 트랜스포트도 정의합니다. 특정 도메인에 대해 전송 제공자나 발신 IP를 조정하여 전달률을 튜닝해야 하는 시나리오가 있을 수 있습니다. 이는 선택 사항입니다.

### DocumentDB

`DocumentDB`에 연결할 때 기본 설정과 호환되도록 `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`을 지정하는 것이 좋습니다.

---