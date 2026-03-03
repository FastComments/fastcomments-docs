[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO는 SSO를 구현하는 메커니즘으로 HMAC-SHA256 암호화를 사용합니다. 먼저 전체 아키텍처를 살펴보고 예제와 상세 단계를 제공합니다.

유사한 SSO 메커니즘을 사용하는 다른 제공업체에서 마이그레이션하는 방법 및 차이점에 관한 문서도 있습니다.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Secure SSO는 풀스택 개발을 포함하므로 Java/Spring, NodeJS/Express, 그리고 순수 PHP의 전체 동작 코드 예제가 현재 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">on GitHub</a>.

NodeJS 예제에서는 ExpressJS를, Java 예제에서는 Spring을 사용했지만 FastComments SSO를 구현하는 데 이 런타임에서 특정 프레임워크나 라이브러리가 필요하지 않습니다 - 네이티브 암호화 패키지로 충분합니다.

FastComments SSO로 새로운 API 엔드포인트를 작성할 필요가 없습니다. 단순히 비밀 키로 사용자의 정보를 암호화하고 페이로드를 댓글 위젯에 전달하면 됩니다.

#### API 시크릿 키 받기

Your API Secret can be retrieved from <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">this page</a>. 이 페이지는 내 계정(My Account)에서 API/SSO 타일을 클릭한 다음 "API 시크릿 키 받기"를 클릭해서도 찾을 수 있습니다.

#### 댓글 위젯 매개변수

댓글 위젯에 대한 상위 수준 API 문서는 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">here</a>에서 찾을 수 있습니다.

이제 이 매개변수들이 무엇을 의미하는지 자세히 살펴보겠습니다.

댓글 위젯은 구성 객체를 사용합니다 - FastComments를 사용하여 고객 id(tenantId)를 전달하고 있다면 이미 이 객체를 전달하고 있는 것입니다.

SSO를 활성화하려면 새로운 "sso" 객체를 전달하세요. 이 객체는 다음 매개변수를 포함해야 합니다. 값들은 서버 측에서 생성되어야 합니다.

- userDataJSONBase64: 사용자의 데이터(JSON 형식)를 Base64로 인코딩한 값.
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64로 생성된 HMAC-SHA256 해시.
- timestamp: Epoch 타임스탬프, **밀리초** 단위. 미래여서는 안 되며, 과거 2일을 초과해서는 안 됩니다.
- loginURL: 댓글 위젯이 사용자 로그인을 위해 표시할 수 있는 URL.
- logoutURL: 댓글 위젯이 사용자 로그아웃을 위해 표시할 수 있는 URL.
- loginCallback: 로그인 URL 대신 제공될 경우, 댓글 위젯이 로그인 버튼을 클릭할 때 호출할 함수.
- logoutCallback: 로그아웃 URL 대신 제공될 경우, 댓글 위젯이 로그아웃 버튼을 클릭할 때 호출할 함수.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### 사용자 객체

[inline-code-attrs-start title = '사용자 객체'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 필수. 최대 1k자. **/
    id: string;
    /** 필수. 최대 1k자. 참고: 고유해야 합니다. **/
    email: string;
    /** 필수. 최대 1k자. 참고: 사용자 이름은 이메일일 수 없습니다. 반드시 고유할 필요는 없습니다. **/
    username: string;
    /** 선택사항. URL의 경우 최대 3k자. 기본값은 이메일 기반의 gravatar입니다. 64로 인코딩된 이미지를 지원하며, 이 경우 제한은 50k자입니다. **/ 
    avatar?: string;
    /** 선택사항. 기본값 false. **/
    optedInNotifications?: boolean;
    /** 선택사항. 기본값 false. **/
    optedInSubscriptionNotifications?: boolean;
    /** 선택사항. 최대 100자. 이 라벨은 이름 옆에 표시됩니다. 해당되는 경우 기본값은 관리자/모더레이터입니다. **/
    displayLabel?: string;
    /** 선택사항. 최대 500자. 사용자 이름 대신 표시됩니다. **/
    displayName?: string;
    /** 선택사항. 최대 2k자. 사용자의 이름이 이 링크로 연결됩니다. **/
    websiteUrl?: string;
    /** 선택사항. 사용자당 최대 100개 그룹. 그룹 id는 50자 이하이어야 합니다. **/
    groupIds?: string[];
    /** 선택사항. 사용자를 관리자임을 표시합니다. **/
    isAdmin?: boolean;
    /** 선택사항. 사용자를 모더레이터임을 표시합니다. **/
    isModerator?: boolean;
    /** 선택사항, 기본값 true. false로 설정하면 사용자 프로필의 "activity" 탭을 활성화합니다. **/
    isProfileActivityPrivate?: boolean;
    /** 선택사항, 기본값 false. true로 설정하면 프로필 댓글을 비활성화합니다. **/
    isProfileCommentsPrivate?: boolean;
    /** 선택사항, 기본값 false. true로 설정하면 이 사용자에게 다이렉트 메시지 전송을 비활성화합니다. **/
    isProfileDMDisabled?: boolean;
    /** 사용자 배지에 대한 선택 구성. **/
    badgeConfig?: {
        /** 할당할 전역 배지 ID 배열. 최대 30개 배지로 제한됩니다. 순서가 유지됩니다. **/
        badgeIds: string[];
        /** 현재 페이지(urlId)에 범위가 지정된 배지 ID 배열. 지정된 페이지에서만 표시됩니다. **/
        pageBadgeIds?: string[];
        /** true이면 기존에 표시된 배지를 교체합니다. 전역 및 페이지 범위 배지는 독립적으로 덮어써집니다. **/
        override?: boolean;
        /** true이면 테넌트 구성에서 배지 표시 속성을 업데이트합니다. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

관리자 및 모더레이터의 경우, `SSOUser` 객체에 각각 `isAdmin` 또는 `isModerator` 플래그를 전달하세요.

#### 알림

알림을 활성화하거나 비활성화하려면 `optedInNotifications` 값을 각각 `true` 또는 `false`로 설정하세요. 사용자가 이 값이 포함된 SSO 페이로드로 페이지를 처음 로드하면 알림 설정이 업데이트됩니다.

또한, 사용자가 구독한 페이지에서의 활동에 대해 (인앱 알림뿐만 아니라) 이메일 알림을 받게 하려면 `optedInSubscriptionNotifications`를 `true`로 설정하세요.

#### VIP 사용자 및 특수 라벨

선택적 "displayLabel" 필드를 사용하여 사용자 이름 옆에 특수 라벨을 표시할 수 있습니다.

#### 인증되지 않은 사용자

인증되지 않은 사용자를 나타내려면 userDataJSONBase64, verificationHash, 또는 timestamp를 채우지 마세요. loginURL을 제공하세요.

이 사용자들은 댓글을 달 수 없으며, 대신 로그인 메시지(구성에 따라 메시지, 링크 또는 버튼)가 표시됩니다.

#### 사용자 데이터 직렬화 및 해싱에 대한 직접 예제

자세한 예제는 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">여기</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">여기</a> (java) 및 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">여기</a> (php)를 참조하세요.

어떤 통합 작업도 복잡하고 어려울 수 있다는 것을 이해합니다. 담당자에게 문의하거나 <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>를 이용하는 것을 주저하지 마세요.