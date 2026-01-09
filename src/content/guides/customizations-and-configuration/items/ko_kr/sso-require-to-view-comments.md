---
FastComments SSO (<a href="#sso">자세한 내용</a>)는 사용자가 다른 플랫폼에 로그인하지 않고도 댓글을 남길 수 있는 방법을 제공합니다.

그러나 이 설정만으로는 댓글 스레드가 안전해지지 않습니다. 기본적으로 댓글 데이터는 공개 정보이므로 페이지를 볼 수 있는 누구나 댓글을 볼 수 있습니다.

설정을 변경하면 관리자나 유효한 SSO 사용자에 의해서만 댓글을 가져오도록 제한할 수 있습니다.

#### 코드 없는 설정

SSO가 설정된 경우, 댓글 스레드의 보기 및 상호작용을 방지하려면 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">사용자 지정 규칙</a>을 생성하면 됩니다.

해당 작업을 수행할 때 SSO를 검색하면 다음 옵션을 찾을 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

해당 옵션을 활성화하고 사용자 지정 규칙을 저장하세요.

#### 특정 도메인 또는 페이지만 보호하기

특정 도메인 또는 페이지만 보호하려면 사용자 지정 규칙을 간단히 구성하면 됩니다.

사용자 지정 UI 상단에서 두 입력 필드, Domain 및 URL ID를 찾을 수 있습니다.

특정 도메인만 보호하려면 해당 도메인을 "domain" 필드에 입력하세요.

특정 페이지를 보호하려면 페이지 URL을 "URL ID" 필드에 입력하세요. FastComments와의 사용자 지정 통합이 있는 경우 URL 대신 여기에서 ID 유형을 입력할 수 있습니다.

#### 보안 수준

SSO를 요구할 때 Simple SSO 또는 Secure SSO 중 어느 것을 요구할지 결정해야 합니다. Simple SSO를 요구하면 둘 다 허용되지만, Secure SSO를 요구하면 해당 콘텐츠는 보기 위해 API key로 해시된 Secure SSO 페이로드로 가져와야 합니다.

보안 수준 옵션은 "Require SSO To View Comments"를 선택하면 표시됩니다.

#### 읽기 이상의 보호

이 옵션을 활성화하면 사용자가 SSO로 로그인하지 않은 경우 해당 페이지나 도메인에 댓글을 달 수 없도록 보호합니다.

#### 주의사항

SSO 통합 이전에 댓글을 작성한 사용자는 SSO 통합을 통해 로그인하지 않는 한 그 댓글을 볼 수 없습니다.

---