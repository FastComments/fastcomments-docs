### 인증된 API 사용 (DefaultApi)

**중요:** 인증된 요청을 하려면 `Configuration` 에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류와 함께 실패합니다.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### 공용 API 사용 (PublicApi)

공용 엔드포인트는 인증이 필요하지 않습니다:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### 중재 대시보드 사용 (ModerationApi)

`ModerationApi`는 중재자 대시보드를 구동합니다. 메서드는 `sso` 토큰을 전달하여 중재자를 대신해 호출됩니다:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO 사용 (Single Sign-On)

SDK에는 보안 SSO 토큰을 생성하기 위한 유틸리티가 포함되어 있습니다:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

간단한 SSO(덜 안전함, 테스트용) 사용 예:

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 공통 문제

1. **401 "missing-api-key" 오류**: DefaultApi 인스턴스를 만들기 전에 `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` 를 설정했는지 확인하세요.
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `DefaultApi`, 클라이언트 측/공용 요청에는 `PublicApi`, 중재 대시보드 요청에는 `ModerationApi` 를 사용하세요.
3. **가져오기 오류**: 올바른 모듈에서 가져오고 있는지 확인하세요:
   - API 클라이언트: `from client import ...`
   - SSO 유틸리티: `from sso import ...`