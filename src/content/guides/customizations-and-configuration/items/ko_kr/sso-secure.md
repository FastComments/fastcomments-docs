[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO는 SSO를 구현하는 메커니즘으로 HMAC-SHA256 암호화를 사용합니다. 먼저 전체 아키텍처를 설명하고, 예제와 세부 단계를 제공하겠습니다.

유사한 SSO 메커니즘을 사용하는 다른 제공업체에서 마이그레이션하는 방법과 차이점에 대한 문서도 일부 있습니다.

플로우는 다음과 같습니다:

<div class="screenshot white-bg">
    <div class="title">보안 SSO 흐름</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="보안 SSO 다이어그램" />
</div>

Secure SSO는 풀스택 개발을 수반하므로, Java/Spring, NodeJS/Express, 순수 PHP의 전체 작동 코드 예제가 현재 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub에</a> 있습니다.

NodeJS 예제에서는 ExpressJS를, Java 예제에서는 Spring을 사용하지만, FastComments SSO를 구현하는 데 런타임에서 프레임워크/라이브러리가 필수는 아니며 네이티브 암호화 패키지로도 작동합니다.

FastComments SSO를 사용한다고 해서 새 API 엔드포인트를 작성할 필요는 없습니다. 단순히 비밀 키로 사용자의 정보를 암호화하고 페이로드를 댓글 위젯에 전달하면 됩니다.

#### Get Your API Secret Key

API Secret은 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">이 페이지</a>에서 가져올 수 있습니다. 이 페이지는 My Account로 이동하여 API/SSO 타일을 클릭한 다음 "Get API Secret Key"를 클릭해서도 찾을 수 있습니다.

#### Comment Widget Parameters

댓글 위젯에 대한 상위 수준의 API 문서는 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">여기</a>에서 확인할 수 있습니다.

이제 이러한 매개변수가 의미하는 바를 더 자세히 살펴보겠습니다.

댓글 위젯은 구성 객체를 사용합니다 - FastComments를 사용하여 고객 ID(tenantId)를 전달하고 있다면 이미 이 구성을 전달하고 있는 것입니다.

SSO를 활성화하려면 새로운 "sso" 객체를 전달하세요. 이 객체는 다음 매개변수를 포함해야 하며, 값은 서버 측에서 생성되어야 합니다.

- userDataJSONBase64: 사용자의 데이터를 JSON 형식으로 표현한 후 Base64로 인코딩한 값입니다.
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64로 생성된 HMAC-SHA256 해시입니다.
- timestamp: 에포크 타임스탬프(밀리초 단위). 미래일 수 없으며, 과거 2일을 초과해서는 안 됩니다.
- loginURL: 댓글 위젯이 사용자에게 로그인을 표시할 수 있는 URL입니다.
- logoutURL: 댓글 위젯이 사용자에게 로그아웃을 표시할 수 있는 URL입니다.
- loginCallback: 로그인 URL 대신 제공되는 경우, 로그인 버튼을 클릭할 때 댓글 위젯이 호출할 함수입니다.
- logoutCallback: 로그아웃 URL 대신 제공되는 경우, 로그아웃 버튼을 클릭할 때 댓글 위젯이 호출할 함수입니다.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = '사용자 객체'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 필수. 최대 1k자. **/
    id: string;
    /** 필수. 최대 1k자. 주의: 고유해야 합니다. **/
    email: string;
    /** 필수. 최대 1k자. 주의: 사용자명은 이메일이 될 수 없습니다. 고유할 필요는 없습니다. **/
    username: string;
    /** 선택사항. URL의 경우 최대 3k자. 기본값은 이메일을 기반으로 한 gravatar입니다. 64인코딩된 이미지를 지원하며, 이 경우 제한은 50k자입니다. **/ 
    avatar?: string;
    /** 선택사항. 기본값: false. **/
    optedInNotifications?: boolean;
    /** 선택사항. 기본값: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** 선택사항. 최대 100자. 이 레이블은 이름 옆에 표시됩니다. 해당되는 경우 기본값은 관리자/모더레이터입니다. **/
    displayLabel?: string;
    /** 선택사항. 최대 500자. 사용자명 대신 표시됩니다. **/
    displayName?: string;
    /** 선택사항. 최대 2k자. 사용자의 이름이 이 링크로 연결됩니다. **/
    websiteUrl?: string;
    /** 선택사항. 사용자당 최대 100개 그룹. 그룹 id는 50자를 초과할 수 없습니다. **/
    groupIds?: string[];
    /** 선택사항. 사용자를 관리자임을 표시합니다. **/
    isAdmin?: boolean;
    /** 선택사항. 사용자를 모더레이터임을 표시합니다. **/
    isModerator?: boolean;
    /** 선택사항, 기본값 true. 사용자 프로필의 '활동' 탭을 활성화하려면 false로 설정하세요. **/
    isProfileActivityPrivate?: boolean;
    /** 선택사항, 기본값 false. 프로필 댓글을 비활성화하려면 true로 설정하세요. **/
    isProfileCommentsPrivate?: boolean;
    /** 선택사항, 기본값 false. 이 사용자에 대한 다이렉트 메시지를 비활성화하려면 true로 설정하세요. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

관리자와 모더레이터의 경우, `SSOUser` 객체에 각각 `isAdmin` 또는 `isModerator` 플래그를 전달하세요.

#### Notifications

알림을 활성화하거나 비활성화하려면 `optedInNotifications` 값을 각각 `true` 또는 `false`로 설정하세요. 사용자가 이 값이 포함된 SSO 페이로드로 페이지를 처음 로드하면 알림 설정이 업데이트됩니다.

또한 사용자가 구독한 페이지의 활동에 대해 (앱 내 알림뿐 아니라) 이메일 알림을 받게 하려면 `optedInSubscriptionNotifications`를 `true`로 설정하세요.

#### VIP Users & Special Labels

선택적 필드인 "displayLabel"을 사용하여 사용자 이름 옆에 특수 레이블을 표시할 수 있습니다.

#### Unauthenticated users

인증되지 않은 사용자를 나타내려면 userDataJSONBase64, verificationHash 또는 timestamp를 채우지 마십시오. 로그인 URL을 제공하세요(loginURL).

이 사용자들은 댓글을 달 수 없으며 대신 로그인 메시지(구성에 따라 메시지, 링크 또는 버튼)가 표시됩니다.

#### Direct Examples for Serializing and Hashing User Data

직렬화 및 사용자 데이터 해싱에 대한 자세한 예제는 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">여기</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">여기</a> (java) 및 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">여기</a> (php)에서 확인할 수 있습니다.

통합 작업이 복잡하고 어려운 과정이 될 수 있다는 점을 이해합니다. 담당자에게 문의하거나 <a href="https://fastcomments.com/auth/my-account/help" target="_blank">지원 페이지</a>를 이용하는 것을 주저하지 마세요.