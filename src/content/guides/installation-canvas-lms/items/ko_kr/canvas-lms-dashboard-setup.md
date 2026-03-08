#### Canvas LTI 구성으로 이동

FastComments 계정에 로그인한 다음 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">내 계정 > Canvas LTI 구성</a>으로 이동합니다.

#### 새로운 LTI 구성 생성

**Enable LTI** 확인란을 선택하세요. 구성 필드가 표시됩니다:

- **Configuration Name** - 이 구성을 식별하기 위한 선택적 레이블(여러 Canvas 인스턴스를 연결하는 경우 유용).
- **Platform URL** - 귀하의 Canvas 인스턴스 URL (예: `https://yourschool.instructure.com`). 현재는 빈 칸으로 두고 Developer Key를 생성한 후 입력할 수 있습니다.
- **Client ID** - 현재는 빈 칸으로 두세요. Canvas에서 Developer Key를 생성한 후 여기에 입력합니다.
- **Deployment ID** - 현재는 빈 칸으로 두세요.
- **Comment Style** - 댓글, Collab 채팅 또는 둘 다 중에서 선택하세요. 자세한 내용은 [댓글 스타일](#canvas-lms-commenting-styles)을 참조하세요.

구성을 생성하려면 **추가**를 클릭하세요.

#### 구성 URL 복사

저장하면 세 개의 URL이 표시됩니다:

- **Configuration URL** - Developer Key를 생성할 때 Canvas에 붙여넣을 URL입니다.
- **OIDC Login URL** - LTI 로그인 흐름에서 Canvas가 사용하는 URL(구성 URL을 통해 자동 구성됨).
- **Launch URL** - 학생이 FastComments를 열 때 Canvas가 호출하는 엔드포인트(구성 URL을 통해 자동 구성됨).

**Configuration URL**을 복사하세요. 다음 단계에서 필요합니다.