FastComments에서 SAML을 구성한 후, IdP(Identity Provider)에서 FastComments를 서비스 제공자(SP)로 설정해야 합니다.

### 일반적인 IdP 구성

대부분의 IdP는 FastComments를 SAML 애플리케이션으로 추가할 때 다음 정보를 요구합니다:

#### 필요한 서비스 제공자 정보

이 값들은 FastComments SAML 구성 페이지에서 자동으로 생성되어 표시됩니다:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- 이 값은 귀하의 FastComments 인스턴스를 고유하게 식별합니다

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- IdP가 인증 후 SAML 응답을 보내는 위치입니다

**SP Metadata URL** *(IdP에서 지원하는 경우)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- XML 형식으로 완전한 SAML 구성을 제공합니다

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- SAML 인증을 시작하는 직접 링크입니다

### 필수 SAML 속성

IdP가 SAML 응답과 함께 다음 속성들을 전송하도록 구성하십시오:

#### 필수 속성

**이메일 주소** *(필수)*
- **속성 이름**: `email`, `emailAddress`, 또는 `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **목적**: 고유 사용자 식별 및 알림
- **형식**: 유효한 이메일 주소

#### 선택적 속성

**이름(First Name)**
- **속성 이름들**: `firstName`, `givenName`, 또는 `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **목적**: 사용자 표시 이름

**성(Last Name)**
- **속성 이름들**: `lastName`, `surname`, 또는 `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **목적**: 사용자 표시 이름

**역할(Roles)** *(접근 제어에 중요)*
- **속성 이름들**: `roles`, `groups`, `memberOf`, 또는 사용자 정의 속성 이름
- **목적**: FastComments의 역할 할당 및 권한 설정
- **형식**: 역할 문자열의 배열 또는 쉼표로 구분된 값

### 일반적인 IdP 구성 사례

#### Microsoft Azure AD

1. **엔터프라이즈 애플리케이션 추가**
   - "FastComments"를 검색하거나 커스텀 SAML 애플리케이션을 만듭니다
   - FastComments에서 제공한 SP 정보를 사용합니다

2. **속성 구성**
   - 이메일: `user.mail` 또는 `user.userprincipalname`
   - 이름: `user.givenname`
   - 성: `user.surname`
   - 역할: `user.assignedroles` 또는 디렉터리 그룹

#### Okta

1. **SAML 애플리케이션 생성**
   - "Create New App"을 사용하고 SAML 2.0을 선택합니다
   - FastComments SP 정보로 구성합니다

2. **속성 선언(Attribute Statements)**
   - 이메일: `user.email`
   - 이름(FirstName): `user.firstName`
   - 성(LastName): `user.lastName`
   - 역할: `user.groups` 또는 사용자 정의 속성

#### Google Workspace

1. **SAML 애플리케이션 추가**
   - Apps > Web and mobile apps > Add App > Add custom SAML app로 이동합니다
   - FastComments SP 정보로 구성합니다

2. **속성 매핑**
   - 이메일: 기본 이메일(Primary email)
   - 이름: First name
   - 성: Last name
   - 역할: 그룹 또는 사용자 정의 속성

#### Active Directory Federation Services (ADFS)

1. **Relying Party Trust 추가**
   - FastComments 메타데이터 URL을 사용하거나 수동으로 구성합니다
   - 제공된 SP 정보를 구성합니다

2. **클레임 규칙(Claim Rules)**
   - 이메일: Email Address 클레임
   - 이름: Name ID 클레임
   - 역할: 그룹 멤버십 또는 사용자 정의 클레임

### 속성 이름의 유연성

FastComments는 다양한 IdP 구성을 수용하기 위해 여러 속성 이름에서 역할 정보를 수락합니다:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

이 유연성은 특정 속성 이름 규칙을 요구하지 않고 다양한 IdP와의 호환성을 보장합니다.

### 구성 테스트

IdP 구성을 완료한 후:

1. IdP 구성을 저장합니다
2. 전용 테스트 사용자 계정으로 테스트합니다
3. 속성이 올바르게 전송되는지 확인합니다
4. 역할이 제대로 매핑되었는지 확인합니다
5. 인증 흐름이 성공적으로 완료되는지 확인합니다

대부분의 IdP는 프로덕션 사용자에게 배포하기 전에 구성을 검증할 수 있는 SAML 테스트 도구를 제공합니다.