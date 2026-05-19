#### Sakai

Sakai는 LTI Advantage가 포함된 릴리스에서 LTI 1.3 동적 등록을 지원합니다. 관리자 워크스페이스에서:

1. Sakai 관리자 계정으로 로그인하고 **관리자 워크스페이스**를 엽니다.
2. **외부 도구** > **LTI 1.3 도구 설치**를 선택합니다.
3. FastComments 등록 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기에서 가져오기</a>)을 붙여넣고 제출합니다.
4. 핸드셰이크가 완료되면 도구를 승인합니다.

그러면 도구가 **외부 도구** 아래에 표시되며 해당 사이트의 유지관리자가 사이트에 추가할 수 있습니다.

#### Schoology

Schoology Enterprise 인스턴스는 LTI 1.3을 지원하지만 동적 등록의 사용 가능 여부는 배포에 따라 다릅니다. Schoology 계정 관리자에게 확인하세요.

동적 등록이 귀하의 Schoology 인스턴스에서 사용 불가능한 경우, 다음 엔드포인트를 사용하여 통합을 수동으로 구성해야 합니다:

- **OIDC 로그인 URL**: `https://fastcomments.com/lti/v1p3/login`
- **대상 링크 URL**: `https://fastcomments.com/lti/v1p3/launch`
- **공개 키셋 URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **리디렉션 URL**: `https://fastcomments.com/lti/v1p3/launch`

Schoology가 Client ID 및 Deployment ID를 제공하면, FastComments 지원팀에 연락하여 테넌트에 구성을 등록해 달라고 요청하세요.

#### Other LTI 1.3 Platforms

IMS LTI 1.3 Advantage 사양을 따르는 모든 LMS는 동일한 등록 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기에서 가져오기</a>)로 작동해야 합니다. "동적 등록", "도구 등록 URL", "도구 시작 등록 엔드포인트" 또는 이와 유사한 레이블이 있는 설정을 찾으십시오.

플랫폼이 수동 LTI 1.3 설정만 지원하는 경우 위 Schoology 섹션에 나열된 네 가지 엔드포인트를 사용하고 최종 구성을 위해 지원팀에 연락하세요.