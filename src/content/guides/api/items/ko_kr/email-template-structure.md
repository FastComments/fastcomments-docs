`EmailTemplate` 객체는 테넌트의 맞춤 이메일 템플릿 구성을 나타냅니다.

시스템은 사용할 이메일 템플릿을 다음 기준으로 선택합니다:

- 유형 식별자 — 이를 `emailTemplateId`라고 합니다. 이 값들은 상수입니다.
- `domain`. 먼저 관련 객체(예: `Comment`)가 속한 도메인에 대한 템플릿을 찾습니다. 일치하는 템플릿이 없으면 domain이 null이거나 `*`인 템플릿을 찾습니다.

다음은 `EmailTemplate` 객체의 구조입니다:

[inline-code-attrs-start title = '이메일 템플릿 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** 읽기 전용 **/
    createdAt: string
    /** 읽기 전용 **/
    updatedAt: string
    /** 읽기 전용 **/
    updatedByUserId: string
    /** 템플릿이 연관된 도메인. **/
    domain?: string | '*' | null
    /** EJS 구문으로 된 이메일 템플릿 콘텐츠. **/
    ejs: string
    /** 지원되는 각 로케일에 대해 재정의된 번역 키와 값의 맵. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** 템플릿의 렌더 컨텍스트를 나타내는 객체. **/
    testData: object
}
[inline-code-end]

### 참고

- 유효한 `emailTemplateId` 값은 `/definitions` 엔드포인트에서 확인할 수 있습니다.
- `/definitions` 엔드포인트에는 기본 번역과 테스트 데이터도 포함되어 있습니다.
- 템플릿은 구조나 테스트 데이터가 유효하지 않으면 저장에 실패합니다.