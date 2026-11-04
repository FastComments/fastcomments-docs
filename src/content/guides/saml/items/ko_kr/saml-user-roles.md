FastComments는 SAML 사용자 역할을 내부 권한에 매핑하여 조직을 위한 역할 기반 접근 제어를 가능하게 합니다.

### FastComments 역할 시스템

FastComments는 역할 기반 권한 시스템을 사용하며, 사용자는 하나 이상의 역할을 가질 수 있고 이 역할이 접근 수준과 기능을 결정합니다.

### 사용 가능한 FastComments 역할

#### 관리 역할

**fc-account-owner**
- **권한**: 전체 관리 접근
- **기능**: 모든 기능, 결제 관리, 사용자 관리
- **사용 사례**: 주 계정 관리자 및 소유자

**fc-admin-admin**  
- **권한**: 대부분의 기능에 대한 관리 접근
- **기능**: 사용자 관리, 구성, 검토. **다른 관리자를 관리할 수 있습니다.**
- **사용 사례**: 보조 관리자 및 IT 담당자

**fc-billing-admin**
- **권한**: 결제 및 구독 관리
- **기능**: 결제 수단, 인보이스, 구독 변경
- **사용 사례**: 재무팀 구성원 및 결제 담당자

#### 전문 역할

**fc-analytics-admin**
- **권한**: 분석 및 보고 접근
- **기능**: 사이트 통계, 사용자 참여 데이터 조회
- **사용 사례**: 마케팅팀 및 데이터 분석가

**fc-api-admin**
- **권한**: API 접근 및 관리
- **기능**: API 자격 증명, 웹훅 구성
- **사용 사례**: 개발자 및 기술 통합 담당자

**fc-moderator**
- **권한**: 댓글 검토 기능
- **기능**: 댓글 승인/거부, 스팸 관리
- **사용 사례**: 커뮤니티 검토자 및 콘텐츠 관리자

### 역할 매핑 구성

#### SAML 속성 소스

FastComments는 다양한 SAML 속성 이름에서 역할 정보를 받아들여 서로 다른 아이덴티티 제공자와의 호환성을 보장합니다:

**표준 속성 이름**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS 속성**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### 역할 형식 지원

**배열 형식** *(권장)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**쉼표로 구분된 형식**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**단일 역할 형식**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### 아이덴티티 제공자 역할 구성

#### Microsoft Azure AD

1. **앱 역할 구성**:
   - Azure AD 애플리케이션에서 FastComments 역할 정의
   - 사용자를 적절한 앱 역할에 할당
   - 할당된 역할을 포함하도록 클레임 구성

2. **속성 매핑**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **그룹 할당**:
   - FastComments 역할 이름과 일치하는 그룹 생성
   - 사용자를 적절한 그룹에 할당
   - 속성 문(attribute statement) 구성

2. **속성 문**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **그룹 매핑**:
   - 조직 단위 또는 그룹 생성
   - FastComments 역할 접두사로 그룹 이름 지정
   - 속성 매핑 구성

2. **커스텀 속성**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### 기본 사용자 동작

#### 역할이 없는 사용자

SAML 사용자에게 역할이 없거나 인식되지 않는 역할이 있는 경우:
- 사용자는 표준 댓글 작성자로 생성됩니다
- 관리 접근 권한이 부여되지 않습니다
- 자신의 댓글을 작성하고 관리할 수 있습니다
- 관리자 대시보드 기능에 접근할 수 없습니다

#### 역할 상속

- 사용자는 여러 역할을 동시에 가질 수 있습니다
- 권한은 누적됩니다 (가장 높은 권한 수준이 적용됨)
- IdP에서의 역할 변경은 다음 로그인 시 반영됩니다

### SAML 사용자 관리

#### 사용자 생성

사용자가 SAML을 통해 처음 로그인할 때:
1. **사용자 계정**: 이메일을 식별자로 하여 자동 생성됩니다
2. **역할 할당**: SAML 속성을 기반으로 역할이 적용됩니다
3. **프로필 정보**: 제공된 경우 이름/성이 채워집니다
4. **권한 활성화**: 역할이 즉시 활성화됩니다

#### 역할 업데이트

기존 SAML 사용자는 역할 업데이트를 받습니다:
1. **로그인 트리거**: 역할 업데이트는 각 SAML 로그인 시 발생합니다
2. **즉시 효과**: 새 권한이 즉시 적용됩니다
3. **역할 제거**: 제거된 역할은 자동으로 취소됩니다
4. **감사 추적**: 역할 변경은 감사 로그에 기록됩니다

### 커스텀 역할 매핑

#### 엔터프라이즈 커스터마이징

특별한 요구 사항이 있는 엔터프라이즈 고객을 위해:
- 커스텀 역할 이름을 FastComments 권한에 매핑할 수 있습니다
- 복잡한 역할 계층 구조를 구현할 수 있습니다
- 부서별 접근 제어를 구성할 수 있습니다

커스텀 역할 매핑 구성에 대해서는 FastComments 지원팀에 문의하세요.

#### 역할 검증

FastComments는 수신된 역할을 검증합니다:
- 인식되지 않는 역할은 무시됩니다 (거부되지 않음)
- 잘못된 형식의 역할 속성은 문제 해결을 위해 기록됩니다
- SAML 어설션에 역할 정보가 없는 경우 사용자는 기존 역할을 유지합니다

### 모범 사례

#### 역할 관리

1. **최소 권한 원칙**: 필요한 최소한의 권한만 할당
2. **정기 감사**: 사용자 역할과 접근 권한을 주기적으로 검토
3. **명확한 명명**: IdP에서 설명적인 그룹 이름 사용
4. **문서화**: 역할 할당에 대한 문서 유지

#### 보안 고려 사항

1. **역할 속성**: SAML 응답에서 역할 속성이 적절히 보호되는지 확인
2. **속성 검증**: 승인된 시스템만 역할을 할당할 수 있는지 확인
3. **접근 검토**: 관리 역할 할당을 정기적으로 검토
4. **모니터링**: 역할 변경 및 관리 작업 모니터링

### 역할 문제 해결

#### 일반적인 문제

**역할이 적용되지 않음**:
- SAML 속성 이름이 지원되는 형식과 일치하는지 확인
- IdP가 역할 정보를 전송하는지 확인
- 역할 값이 FastComments 역할 이름과 정확히 일치하는지 확인

**접근 거부**:
- 사용자가 IdP에서 적절한 역할이 할당되어 있는지 확인
- 역할 철자와 대소문자 구분 확인
- SAML 응답에서 역할이 올바르게 형식화되어 있는지 확인

**누락된 권한**:
- 역할 정의와 필요한 권한 검토
- 충돌하는 역할 할당이 있는지 확인
- 역할 변경 후 사용자가 로그인했는지 확인