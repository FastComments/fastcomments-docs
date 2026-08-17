SAML (Security Assertion Markup Language)은 XML 기반의 개방형 표준으로, 당사자 간, 특히 아이덴티티 제공자(IdP)와 서비스 제공자(SP) 간에 인증 및 권한 부여 데이터를 교환합니다.

### SAML 작동 방식

SAML은 사용자가 아이덴티티 제공자와 한 번 인증하면 여러 애플리케이션에 다시 자격 증명을 입력하지 않고도 접근할 수 있도록 싱글 사인온(SSO)을 가능하게 합니다. 사용자가 FastComments에 접근하려고 할 때:

1. **Authentication Request**: FastComments는 사용자를 귀하의 아이덴티티 제공자로 리디렉션합니다.
2. **User Authentication**: 사용자는 귀하나의 IdP(예: Active Directory, Okta, Azure AD)와 인증합니다.
3. **SAML Response**: IdP는 서명된 SAML 어설션을 FastComments에 다시 보냅니다.
4. **User Access**: FastComments는 어설션을 검증하고 인증된 사용자에게 접근을 허용합니다.

### SAML의 장점

- **Enhanced Security**: 중앙 집중식 인증은 비밀번호와 관련된 보안 위험을 감소시킵니다.
- **Improved User Experience**: 사용자는 한 번 로그인하고 여러 애플리케이션에 원활하게 접근합니다.
- **Compliance**: 접근 제어 및 감사 로그에 대한 규제 요구사항을 충족하는 데 도움이 됩니다.
- **Administrative Control**: IT 관리자는 중앙 집중식 사용자 관리를 유지합니다.

### SAML 2.0 지원

FastComments는 SAML 표준 중 가장 널리 채택된 버전인 SAML 2.0을 구현합니다. 우리의 구현은 다음을 지원합니다:

- HTTP-POST and HTTP-Redirect bindings
- 서명된 SAML 응답 및 어설션
- 암호화된 어설션(옵션)
- 다중 서명 및 다이제스트 알고리즘
- 다양한 이름 식별자 형식