#### Sakai

Sakai는 LTI Advantage가 포함된 릴리스에서 LTI 1.3 Dynamic Registration을 지원합니다. Administration Workspace에서:

1. Sakai 관리자 계정으로 로그인하고 **Administration Workspace**를 엽니다.
2. **External Tools** > **Install LTI 1.3 Tool**을 선택합니다.
3. FastComments 등록 URL을 붙여넣고 제출합니다.
4. 핸드셰이크가 완료되면 도구를 승인합니다.

그 도구는 **External Tools** 아래에 나타나며 사이트 유지관리자가 사이트에 추가할 수 있습니다.

#### Schoology

Schoology Enterprise 인스턴스는 LTI 1.3을 지원하지만 Dynamic Registration의 사용 가능 여부는 배포 방식에 따라 다릅니다. Schoology 계정 관리자에게 확인하십시오.

만약 Dynamic Registration이 Schoology 인스턴스에서 사용 불가능한 경우, 다음 엔드포인트를 사용하여 통합을 수동으로 구성해야 합니다:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology가 Client ID와 Deployment ID를 제공하면, 테넌트에 구성을 등록하기 위해 FastComments 지원팀에 연락하십시오.

#### Other LTI 1.3 Platforms

IMS LTI 1.3 Advantage 사양을 따르는 모든 LMS는 동일한 등록 URL로 작동해야 합니다. "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" 또는 이와 유사한 이름의 설정을 찾으십시오.

플랫폼이 수동 LTI 1.3 설정만 지원하는 경우, 위 Schoology 섹션에 나열된 네 개의 엔드포인트를 사용하고 최종 등록을 위해 지원팀에 연락하십시오.