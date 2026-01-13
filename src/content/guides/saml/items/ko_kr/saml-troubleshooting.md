이 가이드는 일반적인 SAML 인증 문제와 그 해결 방법을 다룹니다.

### Certificate and Security Issues

#### Invalid Certificate Error

**증상**:
- "Certificate validation failed" 오류
- 사용자가 SAML 인증을 완료할 수 없음
- SAML 응답이 거부됨

**일반 원인**:
- 인증서 형식이 올바르지 않음
- 인증서가 만료됨
- 잘못된 인증서가 제공됨
- 인증서에 여분의 문자나 공백이 있음

**해결 방법**:
1. **인증서 형식 확인**:
   - 인증서에 `-----BEGIN CERTIFICATE-----` 및 `-----END CERTIFICATE-----` 마커가 포함되어 있는지 확인
   - 여분의 공백이나 줄 바꿈을 제거
   - 인증서를 IdP 메타데이터나 구성에서 직접 복사

2. **인증서 유효성 검사**:
   - 인증서가 만료되지 않았는지 확인
   - 인증서가 올바른 IdP용인지 확인
   - 온라인 인증서 유효성 검사 도구를 사용하여 형식 확인

3. **인증서 재다운로드**:
   - IdP에서 최신 인증서를 다운로드
   - 가능한 경우 IdP 메타데이터 URL 사용
   - 인증서가 현재 IdP 구성과 일치하는지 확인

#### Signature Verification Failed

**증상**:
- SAML 어설션 서명 검증 오류
- IdP 로그인 후 인증 실패
- "Invalid signature" 오류 메시지

**해결 방법**:
1. **알고리즘 불일치**:
   - FastComments의 서명 알고리즘이 IdP와 일치하는지 확인
   - 다른 서명 알고리즘 시도 (SHA-256, SHA-1, SHA-512)
   - 다이제스트 알고리즘이 IdP 구성과 일치하는지 확인

2. **인증서 문제**:
   - 올바른 서명 인증서가 구성되어 있는지 확인
   - 인증서가 IdP에서 사용되는 개인 키에 해당하는지 검증
   - IdP에서 인증서 교체(로테이션)가 있었는지 확인

### Configuration Issues

#### Wrong Entity ID or ACS URL

**증상**:
- IdP가 "Unknown Service Provider" 보고
- SAML 응답이 잘못된 엔드포인트로 전송됨
- 인증이 완료되지 않음

**해결 방법**:
1. **SP 정보 확인**:
   - FastComments 구성에서 Entity ID를 정확히 복사
   - ACS URL이 다음 형식과 일치하는지 확인: `https://fastcomments.com/saml/callback/{tenant-id}`
   - tenant ID에 오타가 없는지 확인

2. **IdP 구성**:
   - IdP에 올바른 SP Entity ID로 업데이트
   - 올바른 ACS/Reply URL 구성
   - IdP 바인딩 설정 확인 (HTTP-POST 권장)

#### Missing or Incorrect Attributes

**증상**:
- 사용자가 적절한 역할 없이 생성됨
- 사용자 프로필 정보 누락
- "Email required" 오류

**해결 방법**:
1. **이메일 속성**:
   - IdP가 이메일 속성을 전송하는지 확인
   - 속성 이름 매핑 확인 (email, emailAddress 등)
   - 이메일 값이 유효한 이메일 주소인지 검증

2. **역할 속성**:
   - IdP가 역할/그룹 정보를 전송하는지 확인
   - 역할 속성 이름이 FastComments의 기대값과 일치하는지 확인
   - 역할 값이 FastComments 역할 이름과 정확히 일치하는지 확인

3. **속성 형식**:
   - 배열 형식과 쉼표로 구분된 형식 모두 테스트
   - 속성 값에 여분의 공백이 없는지 확인
   - 역할 이름의 대소문자 구분 여부 확인

### Authentication Flow Issues

#### Redirect Loop

**증상**:
- 브라우저가 FastComments와 IdP 사이에서 끝없이 리다이렉트
- 인증이 완료되지 않음
- 브라우저 개발자 도구에 여러 리다이렉트 표시

**해결 방법**:
1. **SP 구성 확인**:
   - Entity ID가 IdP 구성과 정확히 일치하는지 확인
   - ACS URL이 IdP에 올바르게 구성되어 있는지 확인
   - URL에 후행 슬래시가 있는지 확인

2. **세션 문제**:
   - 브라우저 쿠키를 지우고 다시 시도
   - 시크릿/프라이빗 창에서 테스트
   - 세션 타임아웃 설정 확인

#### Access Denied After Authentication

**증상**:
- SAML 인증은 성공함
- 사용자가 FastComments로 리디렉션됨
- "Access denied" 또는 권한 오류 표시

**해결 방법**:
1. **역할 할당**:
   - 사용자가 IdP에서 적절한 역할을 가지고 있는지 확인
   - SAML 응답에 역할 속성이 전송되는지 확인
   - 역할 이름이 FastComments 요구사항과 정확히 일치하는지 확인

2. **패키지 제한**:
   - 계정이 Flex 또는 Pro plan인지 확인
   - 패키지에 SAML 기능이 활성화되어 있는지 확인
   - 패키지에 SAML이 포함되어 있으나 기능을 사용할 수 없는 경우 지원에 문의

### Identity Provider Specific Issues

#### Microsoft Azure AD

**일반 문제**:
- 앱 역할 할당이 토큰에 반영되지 않음
- 클레임이 제대로 전송되지 않음
- 사용자 할당 요구 사항

**해결 방법**:
- FastComments 애플리케이션에 대한 사용자 할당 확인
- 앱 역할이 올바르게 구성되었는지 확인
- 필요한 속성이 포함되도록 클레임 매핑 확인

#### Okta

**일반 문제**:
- 그룹 필터가 제대로 작동하지 않음
- 속성 문(statement)이 잘못 구성됨
- 애플리케이션 할당 문제

**해결 방법**:
- 속성 문 구성 검토
- 그룹 할당 및 필터 규칙 확인
- 애플리케이션이 적절한 사용자/그룹에 할당되어 있는지 확인

#### Google Workspace

**일반 문제**:
- 사용자 정의 속성이 올바르게 매핑되지 않음
- 그룹 구성원이 전송되지 않음
- SAML 애플리케이션 구성 오류

**해결 방법**:
- 역할 속성용 사용자 정의 스키마 구성
- 그룹 구성원 전파 확인
- SAML 애플리케이션 속성 매핑 검증

### Network and Connectivity Issues

#### Timeout Errors

**증상**:
- 인증 프로세스가 시간 초과
- "Request timeout" 또는 유사한 오류
- 인증 흐름이 느림

**해결 방법**:
1. **네트워크 연결성**:
   - 방화벽 규칙이 FastComments 통신을 허용하는지 확인
   - fastcomments.com의 DNS 해석 확인
   - IdP에서 FastComments로의 네트워크 연결성 테스트

2. **성능 문제**:
   - IdP 응답 시간을 확인
   - 인증서 체인 검증이 느리지 않은지 확인
   - IdP와 사용자 간의 네트워크 지연 고려

#### SSL/TLS Issues

**증상**:
- 인증 중 인증서 경고 표시
- SSL 핸드셰이크 실패
- "Secure connection failed" 오류

**해결 방법**:
- 모든 SAML 엔드포인트가 HTTPS를 사용하도록 확인
- 관련된 모든 도메인의 인증서 유효성 확인
- TLS 버전 호환성 확인

### Debugging and Logging

#### Enabling Debug Information

1. **브라우저 개발자 도구**:
   - SAML 흐름 중 Network 탭 모니터링
   - JavaScript 오류는 Console에서 확인
   - SAML POST 요청 검사 (보이는 경우)

2. **IdP 로깅**:
   - IdP에서 SAML 디버깅 활성화
   - SAML 요청/응답 세부 정보를 위해 IdP 로그 검토
   - 속성 매핑 문제 확인

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - SAML이 활성화되지 않았거나 잘못 구성됨
- "Invalid certificate" - 인증서 검증 실패
- "Missing email attribute" - SAML 응답에 필수 이메일이 제공되지 않음

**IdP Logs**:
- "Unknown service provider" - Entity ID 불일치
- "Invalid ACS URL" - Assertion Consumer Service URL이 올바르지 않음
- "User not assigned" - 사용자가 SAML 애플리케이션에 대한 접근 권한이 없음

### Getting Help

#### Information to Gather

지원에 문의할 때 제공할 항목:
- 정확한 오류 메시지와 타임스탬프
- SAML 구성 세부 정보 (민감한 데이터 제외)
- IdP 유형 및 버전
- 문제를 재현하는 단계
- 브라우저 및 네트워크 정보

#### FastComments Support

SAML 관련 문제의 경우:
1. [support portal](https://fastcomments.com/auth/my-account/help) 사용
2. tenant ID 및 영향을 받는 사용자 이메일 포함
3. 오류 메시지 및 구성 세부 정보 제공
4. IdP 유형 및 구성 방식을 명시

#### IdP Support

IdP 특정 문제의 경우:
- SAML 문제 해결을 위해 IdP 문서 참조
- 구성 문제는 IdP 지원 채널 사용
- 일반적인 문제는 IdP 커뮤니티 포럼 활용

### Prevention Tips

#### Best Practices

1. **철저한 테스트**:
   - 비프로덕션 환경에서 구성 변경을 테스트
   - 여러 테스트 사용자로 확인
   - 작동하는 구성 문서화

2. **정기 모니터링**:
   - SAML 인증 실패에 대한 모니터링 설정
   - 인증서 만료일 검토
   - IdP 구성 변경 모니터링

3. **문서화**:
   - SAML 구성 문서 유지
   - 사용자 정의 구성 또는 우회 방법 문서화
   - IdP 관리자 연락처 정보 유지

#### Proactive Maintenance

1. **인증서 관리**:
   - 인증서 만료일 모니터링
   - 인증서 교체(로테이션) 절차 계획
   - 만료 전에 인증서 업데이트 테스트

2. **구성 검토**:
   - 정기적으로 SAML 구성 검토
   - IdP 구성이 최신 상태인지 확인
   - 변경이 있을 때 문서 업데이트