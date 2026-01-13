FastComments는 SSO와 SAML 인증을 모두 제공합니다. 차이점을 이해하면 조직에 적합한 방식을 선택하는 데 도움이 됩니다.

### Simple/Secure SSO 제공

FastComments는 사이트를 통해 댓글 위젯에 인증하기 위한 두 가지 SSO 흐름을 제공합니다.
이는 SAML과 다르며 SAML을 필요로 하지 않습니다. 대신, Simple SSO는 단순히 객체를 댓글 위젯에 전달하는 것을 요구하고, Secure SSO는 여기에 더해 페이로드를 API 키로 해싱합니다.

SAML은 반면에 사용자를 전체 제품(권한에 기반) *as well as* 댓글 위젯에 대해 인증합니다(사용자가 당사 도메인에 대해 서드파티 쿠키를 활성화한 경우).

### SAML Authentication

SAML은 더 강력한 보안 및 통합 기능을 제공하는 엔터프라이즈급 인증 프로토콜입니다:

- **Implementation**: Identity Provider (IdP) 구성 및 인증서 교환 필요
- **Security**: 서명된 XML 어설션을 사용하고 암호화를 지원함
- **Use Case**: 기존 SAML 인프라(Active Directory, Okta 등)를 보유한 기업에 이상적
- **Setup Complexity**: 더 복잡함 - IdP 구성 및 인증서 관리 필요
- **Enterprise Features**: 고급 역할 매핑, 중앙 집중식 사용자 관리, 감사 기록

### When to Choose SAML

조직에서 다음과 같은 경우 SAML 인증을 고려하세요:

- 이미 SAML 호환 Identity Provider(Okta, Azure AD, ADFS 등)를 사용하고 있는 경우
- 엔터프라이즈급 보안 및 규정 준수가 필요한 경우
- 중앙 집중식 사용자 관리 및 접근 제어가 필요한 경우
- 인증에 SAML을 사용하는 여러 응용 프로그램이 있는 경우
- 자세한 감사 기록 및 보안 보고가 필요한 경우

### When to Choose Simple or Secure SSO

우리의 위젯 중심 SSO 솔루션은 다음과 같은 경우 충분할 수 있습니다:

- 커스텀 인증 시스템이 있는 경우
- 최소한의 설정으로 빠른 구현이 필요한 경우
- 엔터프라이즈 아이덴티티 제공자 통합이 필요 없는 경우
- 애플리케이션에서 사용자 데이터를 직접 제어하려는 경우
- 보안 요구사항이 더 단순한 경우

Simple 및 Secure SSO는 사용자가 이미 *귀하의 사이트 또는 앱을 통해* 계정을 가지고 있지만 반드시 SAML을 사용하지는 않는 온라인 포털, 블로그 등에서 일반적으로 사용됩니다.