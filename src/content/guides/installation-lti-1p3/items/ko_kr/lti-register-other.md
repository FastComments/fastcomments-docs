#### Sakai

Sakai는 LTI Advantage가 포함된 릴리스에서 LTI 1.3 Dynamic Registration을 지원합니다. Administration Workspace에서:

1. Sakai 관리자로 로그인하고 **Administration Workspace**를 엽니다.
2. **External Tools** > **Install LTI 1.3 Tool**을 선택합니다.
3. FastComments 등록 URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기서 확인</a>)을 붙여넣고 제출합니다.
4. 핸드셰이크가 완료되면 도구를 승인합니다.

이후 도구는 **External Tools** 아래에 표시되며, 사이트 관리자가 사이트에 추가할 수 있습니다.

#### Schoology

Schoology Enterprise 인스턴스는 LTI 1.3을 지원하지만 Dynamic Registration 사용 가능 여부는 배포 환경에 따라 다릅니다. Schoology 계정 관리자에게 문의하세요.

Schoology 인스턴스에서 Dynamic Registration을 사용할 수 없는 경우, 다음 엔드포인트를 사용하여 통합을 수동으로 구성해야 합니다:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology에서 Client ID와 Deployment ID를 발급받은 후, FastComments 지원팀에 문의하여 해당 테넌트에 구성을 등록하세요.

#### 기타 LTI 1.3 플랫폼

IMS LTI 1.3 Advantage 사양을 따르는 모든 LMS는 동일한 등록 URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기서 확인</a>)로 작동해야 합니다. "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" 또는 유사한 이름의 설정을 찾아보세요.

플랫폼이 수동 LTI 1.3 설정만 지원하는 경우, 위 Schoology 섹션에 나열된 네 가지 엔드포인트를 사용하고 마무리를 위해 지원팀에 문의하세요.