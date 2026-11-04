#### 강의에서 댓글이 표시되는 방식

LTI 통합이 활성화되고 External App이 설치되면, FastComments는 구성한 배치(placement)에 따라 자동으로 작동합니다:

#### 과제 보기 (Assignment View)

**Assignment View** 배치가 활성화되어 있으면, 강의 내 모든 과제 아래에 댓글이 자동으로 표시됩니다. 학생과 강사가 과제를 볼 때 스레드형 댓글 섹션이 나타나며, 과제별로 추가 설정이 필요하지 않습니다.

각 과제는 자체적인 별도의 댓글 스레드를 갖습니다.

#### 리치 콘텐츠 에디터 버튼 (Rich Content Editor Button)

**Editor Button** 배치가 활성화되어 있으면, 강사는 리치 콘텐츠 에디터를 사용하는 모든 콘텐츠에 FastComments를 삽입할 수 있습니다:

1. **Page**, **Quiz**, 또는 **Announcement**를 편집합니다.
2. 리치 콘텐츠 에디터 툴바에서 **FastComments** 버튼을 클릭합니다.
3. FastComments가 콘텐츠에 자동으로 삽입됩니다.
4. 페이지를 저장합니다.

학생이 페이지를 볼 때, 해당 페이지에 고유한 댓글 스레드와 함께 삽입된 FastComments 위젯이 로드됩니다.

#### 자동 SSO

두 배치 모두에서 학생은 Canvas 계정을 통해 자동으로 로그인됩니다. 이름, 이메일, 아바타는 LTI 실행을 통해 동기화되며, 별도의 로그인이 필요하지 않습니다.

#### 공개 접근 차단 (권장)

기본적으로 FastComments 댓글 데이터는 공개적으로 읽을 수 있습니다. 스레드의 URL이나 API 엔드포인트를 추측할 수 있는 사람은 누구나 Canvas 외부에서도 해당 댓글을 볼 수 있습니다. 강의 토론의 경우 수강생만 볼 수 있도록 조회를 제한하는 것이 거의 확실히 바람직합니다.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">위젯 사용자 지정 페이지</a>를 열고 **Require SSO To View Comments**가 활성화된 규칙을 생성한 다음, 보안 수준을 **Secure SSO**로 설정하여 서명된 LTI 실행을 통해서만 스레드를 로드할 수 있도록 하세요.

규칙을 단일 도메인이나 페이지로 범위 지정하는 방법을 포함한 전체 과정은 [Single-Sign-On으로 댓글 스레드 보호](/guide-customizations-and-configuration.html#sso-require-to-view-comments)를 참조하세요.