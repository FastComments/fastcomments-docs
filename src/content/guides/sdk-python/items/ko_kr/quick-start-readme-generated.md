### 인증된 API 사용 (DefaultApi)

**중요:** 인증된 요청을 하기 전에 Configuration에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류로 실패합니다.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API 클라이언트 생성 및 구성
config = Configuration()
config.host = "https://fastcomments.com/api"

# 필수: API 키를 설정하세요 (FastComments 대시보드에서 가져오세요)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# 구성된 클라이언트로 API 인스턴스 생성
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

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # 일반적인 오류:
    # - 401: API 키가 없거나 유효하지 않음
    # - 400: 요청 검증 실패
```

### 퍼블릭 API 사용 (PublicApi)

퍼블릭 엔드포인트는 인증이 필요하지 않습니다:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (싱글 사인온) 사용

SDK에는 안전한 SSO 토큰을 생성하는 유틸리티가 포함되어 있습니다:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 사용자 데이터 생성
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API 시크릿으로 SSO 인스턴스 생성
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# SSO 토큰 생성
sso_token = sso.create_token()

# 이 토큰을 프런트엔드에서 사용하거나 API 호출에 전달하세요
print(f"SSO Token: {sso_token}")
```

간단한 SSO(덜 안전하며 테스트용):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 일반적인 문제

1. **401 "missing-api-key" 오류**: DefaultApi 인스턴스를 생성하기 전에 `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` 를 설정했는지 확인하세요.
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `DefaultApi`를 사용하고, 클라이언트 측/퍼블릭 요청에는 `PublicApi`를 사용하세요.
3. **임포트 오류**: 올바른 모듈에서 임포트하고 있는지 확인하세요:
   - API 클라이언트: `from client import ...`
   - SSO 유틸리티: `from sso import ...`