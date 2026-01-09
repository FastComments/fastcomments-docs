FastComments에서 SAML 인증을 설정하려면 관리자 대시보드에서의 구성과 IdP(아이덴티티 제공자) 쪽 설정이 모두 필요합니다.

### 사전 요구 사항

SAML을 구성하기 전에 다음이 있는지 확인하세요:

- FastComments Flex 또는 Pro 플랜 (Creators 플랜에서는 SAML을 사용할 수 없습니다)
- FastComments 계정에 대한 관리자 권한
- 아이덴티티 제공자에 대한 관리자 권한
- IdP의 SAML 메타데이터 또는 인증서 정보

### SAML 구성 접근

1. [FastComments 관리자 대시보드](https://fastcomments.com/auth/my-account)에 로그인합니다
2. 왼쪽 사이드바에서 **API/SSO 설정**으로 이동합니다
3. **SAML Config** 버튼을 클릭합니다

SAML Config 버튼이 보이지 않는 경우 다음을 확인하세요:
- 귀하의 계정에 필요한 패키지(Flex 또는 Pro)가 포함되어 있는지
- 귀하에게 관리자 권한이 있는지
- 귀하의 사용자에게 API Admin 또는 Admin Admin 역할이 있는지

### 기본 SAML 구성

#### SAML 인증 활성화

1. **SAML 인증 활성화** 체크박스를 선택합니다
2. 이렇게 하면 테넌트에 SAML이 활성화되고 구성 필드가 사용 가능해집니다

#### 필수 필드

**IdP Single Sign-On URL** *(필수)*
- 사용자가 인증을 위해 리디렉션될 URL
- 일반적으로 아이덴티티 제공자가 제공합니다
- 예시: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(필수)*
- 아이덴티티 제공자의 공개 인증서
- SAML 응답의 진위를 검증하는 데 사용됩니다
- BEGIN/END 마커가 포함된 전체 인증서를 포함해야 합니다
- 예시 형식:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### 선택적 필드

**IdP Entity ID / Issuer**
- 아이덴티티 제공자를 식별합니다
- 비워 두면 기본값은 귀하의 FastComments URL입니다
- IdP에 구성된 발급자(issuer)와 일치해야 합니다

### 고급 구성

#### 보안 설정

**Signature Algorithm**
- 기본값은 SHA-256(권장)
- 옵션: SHA-1, SHA-256, SHA-512
- IdP의 구성과 일치해야 합니다

**Digest Algorithm**
- 기본값은 SHA-256(권장)
- SAML 응답의 다이제스트 계산에 사용됩니다
- IdP의 구성과 일치해야 합니다

**Name ID Format**
- 기본값은 이메일 주소 형식
- 사용자 식별자가 어떻게 포맷되는지를 결정합니다
- 일반적인 옵션: 이메일 주소, 영구(Persistent), 일시적(Transient)

#### 암호화(선택 사항)

**복호화를 위한 개인 키**
- IdP가 SAML 어설션을 암호화하는 경우에만 필요합니다
- 복호화에 사용되는 개인 키를 붙여넣으세요
- 대부분의 배포에서는 어설션 암호화가 필요하지 않습니다

### 구성 저장

1. 모든 설정이 정확한지 검토합니다
2. **Save SAML Configuration**을 클릭합니다
3. 시스템이 구성을 검증합니다
4. 성공하면 확인 메시지가 표시됩니다

### 다음 단계

FastComments SAML 구성을 저장한 후:

1. 서비스 제공자(Service Provider) 정보를 사용하여 아이덴티티 제공자를 구성합니다
2. 인증 흐름을 테스트합니다
3. 필요에 따라 사용자 역할 및 권한을 설정합니다

IdP 구성을 위해 필요한 서비스 제공자 정보는 SAML이 활성화되면 표시됩니다.