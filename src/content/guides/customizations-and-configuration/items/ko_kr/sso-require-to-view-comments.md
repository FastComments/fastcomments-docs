FastComments SSO (<a href="#sso">자세히 보기</a>)는 사용자가 다른 플랫폼에 로그인하지 않고도 댓글을 달 수 있는 방법을 제공합니다.

하지만 이것만으로는 댓글 스레드를 보호하지 못합니다. 기본적으로 댓글 데이터는 공개 정보이므로 페이지를 볼 수 있는 사람은 누구든지 댓글을 볼 수 있습니다.

설정을 변경하면 관리자가 아니거나 유효한 SSO 사용자가 아닌 경우 댓글을 가져오는 것을 제한할 수 있습니다.

#### 코드 없이 설정

SSO가 설정된 경우 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">맞춤 규칙</a>을 생성하여 댓글 스레드의 조회 및 상호작용을 방지할 수 있습니다.

이때 SSO를 검색하면 다음 옵션을 찾을 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='맞춤 규칙에서 보안 수준 선택과 함께 댓글 보기를 위해 SSO 요구 옵션이 활성화된 모습'; title='댓글 보기를 위한 SSO 요구' app-screenshot-end]

이를 활성화하고 맞춤 규칙을 저장합니다.

#### 특정 도메인 또는 페이지만 보호하기

특정 도메인이나 페이지만 보호하려면 맞춤 규칙을 해당하도록 구성하면 됩니다.

맞춤 UI 상단에 도메인과 URL ID 두 개의 입력란이 있습니다.

특정 도메인만 보호하려면 해당 도메인을 "domain" 필드에 입력합니다.

특정 페이지를 보호하려면 "URL ID" 필드에 페이지 URL을 입력합니다. FastComments와 맞춤형 통합을 사용 중이라면 URL 대신 ID 유형을 입력할 수 있습니다.

#### 보안 수준

SSO를 요구할 때 Simple SSO와 Secure SSO 중 어떤 것을 요구할지 결정해야 합니다. Simple SSO를 요구하면 두 가지 모두 허용되지만, Secure SSO를 요구하면 콘텐츠를 보기 위해 API 키로 해시된 Secure SSO 페이로드를 사용해 가져와야 합니다.

"댓글 보기를 위한 SSO 요구"를 선택하면 보안 수준 옵션이 표시되며, 해당 옵션이 켜져 있는 동안에만 적용됩니다.

<a href="#sso-simple">Simple SSO를 통한 자동 댓글 검증 및 Simple SSO 관리자 허용</a> 설정은 별도로 존재하며, 이 옵션의 켜짐 여부와 관계없이 적용됩니다.

#### 읽기 이상의 보호

이 옵션을 활성화하면 사용자가 SSO를 통해 로그인하지 않는 한 해당 페이지나 도메인에 댓글을 달 수 없게 보호합니다.

#### 주의사항

SSO 통합 이전에 댓글을 만든 사용자는 SSO 통합을 통해 로그인하지 않으면 해당 댓글을 볼 수 없습니다.