FastComments에서 SAML이 활성화되면, 시스템은 IdP(Identity Provider)에 구성해야 하는 서비스 제공자(SP) 정보를 자동으로 생성합니다.

### 서비스 제공자 정보에 접근하기

SP 정보는 SAML 인증을 활성화한 후 SAML 구성 페이지에 표시됩니다. 이 정보에는 IdP가 SAML 신뢰 관계를 설정하는 데 필요한 모든 세부 정보가 포함됩니다.

### 서비스 제공자 엔드포인트

#### SP 엔터티 ID / Audience
**목적**: FastComments 인스턴스를 서비스 제공자로서 고유하게 식별합니다  
**형식**: `https://fastcomments.com/saml/{your-tenant-id}`  
**사용법**: IdP에서 엔터티 ID 또는 Audience로 구성하세요

이 식별자는 SAML 응답이 특정 FastComments 테넌트용임을 보장하여 다른 인스턴스에서 SAML 응답이 수락되는 것을 방지합니다.

#### Assertion Consumer Service (ACS) URL
**목적**: 사용자가 인증된 후 IdP가 SAML 응답을 전송하는 엔드포인트입니다  
**형식**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**사용법**: IdP에서 ACS URL 또는 Reply URL로 구성하세요

이곳은 사용자가 IdP에서 인증에 성공한 후 리디렉션되는 위치이며, 사용자 정보를 포함한 SAML 어설션이 함께 전송됩니다.

#### SP 메타데이터 URL
**목적**: 표준 XML 형식으로 완전한 SAML 구성을 제공합니다  
**형식**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**사용법**: 일부 IdP는 이 URL을 사용하여 구성을 자동으로 가져올 수 있습니다

메타데이터 URL에는 XML 형식의 모든 필요한 SP 정보가 포함되어 있어 호환되는 IdP를 자동으로 구성하기 쉽습니다.

#### SAML 로그인 URL
**목적**: 테넌트에 대해 SAML 인증을 시작하는 직접 링크입니다  
**형식**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**사용법**: 사용자를 SAML 인증으로 직접 연결하거나 흐름을 테스트할 때 사용하세요

이 URL을 사용하여 SAML 인증을 테스트하거나 사용자가 SAML을 통해 로그인할 수 있도록 직접 링크를 제공할 수 있습니다.

### SAML 바인딩 지원

FastComments는 다음 SAML 바인딩을 지원합니다:

#### HTTP-POST 바인딩
- **기본 방법**: SAML 응답에 가장 일반적으로 사용되는 바인딩  
- **보안**: SAML 응답이 HTTP POST를 통해 ACS URL로 전송됩니다  
- **사용법**: 프로덕션 배포에 권장됩니다

#### HTTP-Redirect 바인딩
- **대체 방법**: SAML 응답이 HTTP 리디렉트를 통해 전송됩니다  
- **제한사항**: URL 길이 제한으로 인해 페이로드 크기가 제한됩니다  
- **사용법**: 지원되지만 HTTP-POST가 권장됩니다

### Name ID 정책

FastComments는 SAML 요청에서 다음 Name ID 정책을 구성합니다:

- **기본 형식**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **대체 형식**: Persistent, Transient, Unspecified (구성 가능)  
- **요구사항**: 이메일 주소가 기본 사용자 식별자로 사용됩니다

### SAML 요청 속성

SAML 인증을 시작할 때 FastComments는 다음과 같은 특성을 가진 요청을 전송합니다:

#### 요청 서명
- **상태**: 선택 사항(구성 가능)  
- **알고리즘**: 구성된 서명 알고리즘과 일치  
- **인증서**: 요청 서명이 활성화된 경우 테넌트별 인증서를 사용

#### 요청된 속성
FastComments는 SAML AuthnRequests에서 다음 속성들을 요청합니다:

- **이메일**: 사용자 식별을 위해 필수  
- **이름(First Name)**: 표시 목적의 선택 사항  
- **성(Last Name)**: 표시 목적의 선택 사항  
- **역할/그룹(Roles/Groups)**: 접근 제어 및 권한을 위한 선택 사항

### SP 정보 복사

SAML 구성 페이지는 SP 정보를 클립보드로 자동 복사하는 클릭 가능한 필드를 제공합니다:

1. 엔터티 ID, ACS URL 등 임의의 SP 정보 필드를 클릭합니다  
2. 값이 자동으로 클립보드에 복사됩니다  
3. 값을 IdP 구성에 붙여넣습니다  
4. 짧은 하이라이트가 복사 성공을 표시합니다

이를 통해 오타 없이 SP 정보를 IdP로 정확하게 전송할 수 있습니다.

### SP 인증서 정보

#### 인증서 사용
- **목적**: 통신을 암호화하고 SP 신원을 검증합니다  
- **회전(갱신)**: 인증서는 FastComments에 의해 자동으로 관리됩니다  
- **접근**: 공개 인증서는 메타데이터 URL을 통해 제공됩니다

#### 인증서 세부 정보
- **알고리즘**: RSA-2048 이상  
- **유효성**: 인증서는 만료 전에 자동으로 갱신됩니다  
- **배포**: 표준 SAML 메타데이터를 통해 제공됩니다

### SP 구성 문제 해결

IdP가 SP 정보와 관련한 문제를 보고할 경우:

1. **URL 확인**: 모든 URL이 HTTPS를 사용하고 올바른 테넌트 ID를 포함하는지 확인하세요  
2. **메타데이터 확인**: 메타데이터 URL을 사용하여 구성을 검증하세요  
3. **연결성 테스트**: IdP가 FastComments 엔드포인트에 접근할 수 있는지 확인하세요  
4. **형식 검증**: IdP가 SP 정보 형식을 지원하는지 확인하세요

일반적인 문제는 다음과 같습니다:
- URL에 잘못된 테넌트 ID가 포함된 경우  
- IdP와 FastComments 간의 네트워크 연결 문제  
- IdP가 다른 URL 형식이나 추가 구성 옵션을 기대하는 경우