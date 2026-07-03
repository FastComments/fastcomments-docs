### Using Authenticated APIs (DefaultApi)

**중요:** 인증된 요청을 하기 전에 Configuration에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류와 함께 실패합니다.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API 클라이언트를 생성하고 구성합니다
config = Configuration()
config.host = "https://fastcomments.com"

# 필수: API 키 설정 (FastComments 대시보드에서 가져오세요)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# 구성된 클라이언트로 API 인스턴스를 생성합니다
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# 이제 인증된 API 호출을 할 수 있습니다
try:
    # 예시: SSO 사용자 추가
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # 일반적인 오류:
    # - 401: API 키가 없거나 잘못되었습니다
    # - 400: 요청 검증 실패
```

### Using Public APIs (PublicApi)

공용 엔드포인트는 인증이 필요하지 않습니다:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using the Moderation Dashboard (ModerationApi)

`ModerationApi`는 모더레이터 대시보드를 구동합니다. 메서드는 `sso` 토큰을 전달하여 모더레이터를 대신해 호출됩니다:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # 검토 대기 중인 댓글 수를 가져옵니다
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

SDK에는 보안 SSO 토큰을 생성하는 유틸리티가 포함되어 있습니다:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 사용자 데이터 생성 (id, email, username은 필수입니다)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API 비밀키 (HMAC-SHA256)로 서명합니다
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# 위젯이나 API 호출에 사용할 SSO 토큰을 생성합니다
sso_token = sso.create_token()

# 이 토큰을 프런트엔드에서 사용하거나 API 호출에 전달합니다
print(f"SSO Token: {sso_token}")
```

간단한 SSO(덜 안전, 테스트용) 사용 예시:

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **401 "missing-api-key" 오류**: DefaultApi 인스턴스를 만들기 전에 `config.api_key = {"api_key": "YOUR_KEY"}`를 설정했는지 확인하십시오.
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `DefaultApi`를, 클라이언트 측/공용 요청에는 `PublicApi`를, 모더레이터 대시보드 요청에는 `ModerationApi`를 사용하십시오.
3. **Import 오류**: 올바른 모듈에서 가져오는지 확인하십시오:
   - API 클라이언트: `from client import ...`
   - SSO 유틸리티: `from sso import ...`