#### 코스에서 댓글이 나타나는 방법

LTI 통합이 활성화되고 External App이 설치되면, FastComments는 구성한 placements에 따라 자동으로 작동합니다:

#### Assignment View

**Assignment View** 배치가 활성화된 경우, 코스의 모든 과제 아래에 댓글이 자동으로 표시됩니다. 학생과 강사는 과제를 볼 때 스레드 형식의 댓글 섹션을 보게 되며 — 과제별로 추가 설정이 필요하지 않습니다.

각 과제는 별도의 댓글 스레드를 갖습니다.

#### Rich Content Editor Button

**Editor Button** 배치가 활성화된 경우, 강사는 Rich Content Editor를 사용하는 모든 콘텐츠에 FastComments를 삽입할 수 있습니다:

1. **Page**, **Quiz**, 또는 **Announcement**를 편집합니다.
2. Rich Content Editor 도구 모음에서 **FastComments** 버튼을 클릭합니다.
3. FastComments가 콘텐츠에 자동으로 삽입됩니다.
4. 페이지를 저장합니다.

학생이 페이지를 볼 때, 삽입된 FastComments 위젯이 해당 페이지 고유의 댓글 스레드와 함께 로드됩니다.

#### 자동 SSO

두 배치 모두에서 학생들은 Canvas 계정을 통해 자동으로 로그인됩니다. 이름, 이메일, 아바타는 LTI 런치를 통해 동기화되며 별도의 로그인은 필요 없습니다.

#### 공개 접근 제한 (권장)

기본적으로 FastComments의 댓글 데이터는 공개적으로 읽을 수 있습니다. 스레드의 URL이나 API endpoint를 추측할 수 있는 사람은 Canvas 외부에서도 해당 댓글을 볼 수 있습니다. 코스 토론의 경우 거의 확실히 수강생으로 한정해서 보기 권한을 제한하길 원할 것입니다.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">위젯 맞춤 설정 페이지</a>를 열고 **Require SSO To View Comments**가 활성화된 규칙을 만든 다음 보안 수준을 **Secure SSO**로 설정하여 스레드가 서명된 LTI 런치를 통해서만 로드되도록 하세요.

전체 절차(특정 도메인이나 페이지로 규칙 범위를 제한하는 방법 포함)는 [단일 로그인(SSO)으로 댓글 스레드 보호](/guide-customizations-and-configuration.html#sso-require-to-view-comments)를 참조하세요.