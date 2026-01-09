---
FastComments는 SAML 사용자 역할을 내부 권한에 매핑하여 조직을 위한 역할 기반 접근 제어를 제공합니다.

### FastComments Role System

FastComments는 사용자가 하나 이상의 역할을 가질 수 있는 역할 기반 권한 시스템을 사용하여 접근 수준과 기능을 결정합니다.

### Available FastComments Roles

#### Administrative Roles

**fc-account-owner**
- **권한**: 전체 관리자 접근
- **기능**: 모든 기능, 청구 관리, 사용자 관리
- **사용 사례**: 기본 계정 관리자 및 소유자

**fc-admin-admin**  
- **권한**: 대부분 기능에 대한 관리자 접근
- **기능**: 사용자 관리, 구성, 모더레이션. **다른 관리자도 관리할 수 있습니다.**
- **사용 사례**: 보조 관리자 및 IT 담당자

**fc-billing-admin**
- **권한**: 청구 및 구독 관리
- **기능**: 결제 수단, 인보이스, 구독 변경
- **사용 사례**: 재무 팀 구성원 및 청구 담당자

#### Specialized Roles

**fc-analytics-admin**
- **권한**: 분석 및 보고서 접근
- **기능**: 사이트 통계 보기, 사용자 참여 데이터
- **사용 사례**: 마케팅 팀 및 데이터 분석가

**fc-api-admin**
- **권한**: API 접근 및 관리
- **기능**: API 자격 증명, 웹훅 구성
- **사용 사례**: 개발자 및 기술 통합 담당자

**fc-moderator**
- **권한**: 댓글 모더레이션 기능
- **기능**: 댓글 승인/거부, 스팸 관리
- **사용 사례**: 커뮤니티 모더레이터 및 콘텐츠 관리자

### Role Mapping Configuration

#### SAML Attribute Sources

FastComments는 다양한 아이덴티티 공급자와의 호환성을 보장하기 위해 여러 SAML 속성 이름에서 역할 정보를 수락합니다:

**표준 속성 이름**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS 속성**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Role Format Support

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

### Identity Provider Role Configuration

#### Microsoft Azure AD

1. **앱 역할 구성**:
   - Azure AD 애플리케이션에 FastComments 역할 정의
   - 적절한 앱 역할에 사용자 할당
   - 할당된 역할을 포함하도록 클레임 구성

2. **속성 매핑**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **그룹 할당**:
   - FastComments 역할 이름과 일치하는 그룹 생성
   - 적절한 그룹에 사용자 할당
   - 속성 문(statement) 구성

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

2. **사용자 지정 속성**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Default User Behavior

#### Users Without Roles

SAML 사용자가 역할이 없거나 인식되지 않는 역할일 때:
- 사용자는 일반 댓글 작성자로 생성됩니다
- 관리자 접근 권한은 부여되지 않습니다
- 자신의 댓글을 게시하고 관리할 수 있습니다
- 관리자 대시보드 기능에 접근할 수 없습니다

#### Role Inheritance

- 사용자는 동시에 여러 역할을 가질 수 있습니다
- 권한은 누적됩니다 (가장 높은 권한 수준이 적용됩니다)
- IdP의 역할 변경은 다음 로그인 시 반영됩니다

### Managing SAML Users

#### User Creation

사용자가 SAML로 처음 로그인할 때:
1. **사용자 계정**: 이메일을 식별자로 하여 자동 생성됩니다
2. **역할 할당**: SAML 속성에 따라 역할이 적용됩니다
3. **프로필 정보**: 제공된 경우 이름/성(First/last name)이 채워집니다
4. **권한 활성화**: 역할은 즉시 활성화됩니다

#### Role Updates

기존 SAML 사용자는 역할 업데이트를 받습니다:
1. **로그인 트리거**: 역할 업데이트는 각 SAML 로그인 중에 발생합니다
2. **즉시 적용**: 새 권한은 즉시 적용됩니다
3. **역할 제거**: 제거된 역할은 자동으로 해제됩니다
4. **감사 추적**: 역할 변경은 감사 로그에 기록됩니다

### Custom Role Mapping

#### Enterprise Customization

특정 요구 사항이 있는 엔터프라이즈 고객의 경우:
- 맞춤 역할 이름을 FastComments 권한에 매핑할 수 있습니다
- 복잡한 역할 계층 구조를 구현할 수 있습니다
- 부서별 접근 제어를 구성할 수 있습니다

맞춤 역할 매핑 구성을 위해 FastComments 지원팀에 문의하세요.

#### Role Validation

FastComments는 수신된 역할을 검증합니다:
- 인식되지 않는 역할은 무시됩니다(거부되지 않음)
- 잘못된 형식의 역할 속성은 문제 해결을 위해 기록됩니다
- SAML 어서션에 역할 정보가 없을 경우 사용자는 기존 역할을 유지합니다

### Best Practices

#### Role Management

1. **최소 권한 원칙**: 필요한 최소 권한만 할당하세요
2. **정기 감사**: 사용자 역할과 접근을 정기적으로 검토하세요  
3. **명확한 명명**: IdP에서 설명적인 그룹 이름을 사용하세요
4. **문서화**: 역할 할당에 대한 문서를 유지하세요

#### Security Considerations

1. **역할 속성**: SAML 응답에서 역할 속성이 적절히 보호되는지 확인하세요
2. **속성 검증**: 승인된 시스템만 역할을 할당할 수 있는지 검증하세요
3. **접근 검토**: 관리자 역할 할당을 정기적으로 검토하세요
4. **모니터링**: 역할 변경 및 관리자 작업을 모니터링하세요

### Troubleshooting Role Issues

#### Common Problems

**Roles Not Applied**:
- SAML 속성 이름이 지원되는 형식과 일치하는지 확인하세요
- IdP가 역할 정보를 전송하고 있는지 확인하세요
- 역할 값이 FastComments 역할 이름과 정확히 일치하는지 확인하세요

**Access Denied**:
- IdP에서 사용자에게 적절한 역할이 할당되었는지 확인하세요
- 역할 철자와 대소문자를 확인하세요
- SAML 응답에서 역할이 올바르게 형식화되었는지 확인하세요

**Missing Permissions**:
- 역할 정의와 필요한 권한을 검토하세요
- 상충되는 역할 할당이 있는지 확인하세요
- 역할 변경 후 사용자가 로그인했는지 확인하세요

---