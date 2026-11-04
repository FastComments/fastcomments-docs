FastComments는 설정을 위해 환경 변수를 사용합니다. 다음 목록은 On-Prem과 관련된 지원되는 모든 변수를 정리한 것입니다.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | 환경 유형.                                                                                                                                  | Yes      | production, dev                                       |
| MONGO_URI                      |                             | DB 연결 URI.                                                                                                                                 | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | 데이터베이스 연결에 SSL 사용을 활성화합니다.                                                                                                      | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Mongo에 연결할 때 CA에 대한 인증서 검증을 활성화합니다.                                                                               | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem 파일.                                                                                                                             | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | 중요한 시스템 관련 알림을 받을 이메일 주소.                                                                                      | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | IP 주소 해싱에 사용되는 솔트.                                                                                                                     | Yes      |                                                       |
| SESSION_SECRET                 |                             | 세션 서명에 사용되는 키.                                                                                                                     | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | 저장소의 세션 서명/해싱에 사용되는 키. SESSION_SECRET과 달라야 합니다.                                                              | Yes      |                                                       |
| HOSTNAME                       |                             | FastComments가 배포된 호스트명 (관리자 대시보드 등). 포트나 프로토콜은 포함하면 안 됩니다.                                            | Yes      | example.com                                           |
| HOST_ADDR                      |                             | FastComments가 배포된 접근 가능한 URI (관리자 대시보드 등).                                                                            | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | 이메일 설정(SMTP, 도메인/제공자 매핑 등)이 위치한 로컬 파일 시스템 경로.                                                   | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | 이메일 "From Name" 헤더.                                                                                                                          | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | 이메일 푸터 로고.                                                                                                                                 | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | EMAIL_CONFIG_PATH의 "defaultTransport"에 대한 재정의. 동일한 설정 파일을 다른 환경에 배포할 때 유용합니다.                                     | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | fastcomments.com에서의 계정 ID. 라이선스 키 등록에 사용됩니다.                                                                     | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-prem 라이선스 키.                                                                                                                            | No       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API 키. 지정하지 않으면 gif 선택기를 비활성화하는 설정 규칙을 생성해야 합니다.                                                     | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | giphy 통합에 사용됩니다. 위젯 커스터마이징 규칙으로 재정의할 수도 있습니다.                                                                | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | 선택적 GPT 기반 스팸 감지와 같은 openai 기반 기능에 사용됩니다.                                                                           | No       |                                                       |
| CDN_HOST_ADDR                  |                             | 에셋을 가져올 호스트명. 기본값은 HOSTNAME 값입니다.                                                                     | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | 대용량 파일(내보내기 등)을 가져올 호스트명. 기본값은 CDN_HOST_ADDR 값입니다.                                                | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | 내보내기 같은 대용량 파일을 저장할 위치.                                                                                                 | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | 이메일을 발송할 호스트명.                                                                                                     | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments 쿠키의 이름.                                                                                                                             | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | 쿠키의 "hostname" 필드 값. 접두사로 점(dot)을 붙이는 것이 권장됩니다.                                                                          | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다. 정의되지 않은 경우 로컬 FS가 기본값입니다.                                                                       | No       |                                                       |
| S3_SECRET_KEY                  |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | No       |                                                       |
| S3_REGION                      |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | No       |                                                       |
| S3_BUCKET                      |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | No       |                                                       |
| S3_HOST                        |                             | 사용자 파일 업로드, 아바타 등에 사용됩니다.                                                                                                          | No       |                                                       |
| CACHE_DIR                      |                             | DB를 사용할 수 없을 때를 위한 선택적 오프라인 캐시 저장 위치. 상위 100개 댓글 스레드로 주기적으로 갱신됩니다.                       | No       |                                                       |
| BACKUP_DIR                     |                             | DB를 사용할 수 없을 때를 대비한 데이터 저장 위치. DB를 사용할 수 없을 때 댓글이 제출되면 여기에 저장되며 나중에 처리됩니다. | No       |                                                       |

모든 도메인 관련 변수는 `_HOST` 또는 `_ADDR` 접미사를 사용합니다. 차이점은 다음과 같습니다:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH`에는 다음 예시 형식의 JSON 파일 경로가 포함되어야 합니다:

[inline-code-attrs-start title = '이메일 설정'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

위 예시에서는 `mailgun`이라는 기본 `SMTP` 이메일 전송 방식을 정의합니다. 또한 `@yahoo.com` 이메일 전용으로 사용하는 특수 전송 방식도 정의합니다. 특정 도메인에 대해 전달률을 조정하기 위해 특정 제공자나 발송 IP를 사용하는 것이 바람직한 경우가 있습니다. 이는 선택 사항입니다.

### DocumentDB

`DocumentDB`에 연결할 때는 기본 설정과 호환되도록 `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`을 지정해야 합니다.