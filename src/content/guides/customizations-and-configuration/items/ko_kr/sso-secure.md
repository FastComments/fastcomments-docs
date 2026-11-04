[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO는 SSO를 구현하기 위한 메커니즘으로 HMAC-SHA256 암호화를 사용합니다. 먼저 전체 아키텍처를 살펴본 후, 예제와 상세한 단계를 제공합니다.

또한 유사한 SSO 메커니즘을 가진 다른 공급자로부터 마이그레이션하는 방법과 그 차이점에 대한 문서도 있습니다.

흐름은 다음과 같습니다:

<div class="screenshot white-bg">
    <div class="title">Secure SSO 흐름</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO 다이어그램" />
</div>

Secure SSO는 풀스택 개발이 필요하므로, Java/Spring, NodeJS/Express, 그리고 순수 PHP로 작성된 완전히 동작하는 코드 예제가 현재 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub</a>에 있습니다.

NodeJS 예제에서는 ExpressJS를, Java 예제에서는 Spring을 사용하지만, FastComments SSO를 구현하는 데 이러한 런타임에서 프레임워크/라이브러리가 필요하지 않습니다 - 기본 암호화 패키지로 충분합니다.

FastComments SSO를 사용하면 새로운 API 엔드포인트를 작성할 필요가 없습니다. 시크릿 키를 사용하여 사용자 정보를 암호화하고 페이로드를 댓글 위젯에 전달하기만 하면 됩니다.

#### API 시크릿 키 발급 받기

API 시크릿은 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">이 페이지</a>에서 확인할 수 있습니다. My Account로 이동하여 API/SSO 타일을 클릭한 다음 "Get API Secret Key"를 클릭해서도 이 페이지를 찾을 수 있습니다.

#### 댓글 위젯 파라미터

댓글 위젯에 대한 상위 수준 API 문서는 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">여기</a>에서 확인할 수 있습니다.

이러한 파라미터의 의미를 좀 더 자세히 살펴보겠습니다.

댓글 위젯은 설정 객체를 받습니다 - FastComments를 사용하여 고객 ID(tenantId라고 함)를 전달하고 있다면 이미 이 객체를 전달하고 있는 것입니다.

SSO를 활성화하려면 다음 파라미터를 포함해야 하는 새로운 "sso" 객체를 전달합니다. 값은 서버 측에서 생성되어야 합니다.

- userDataJSONBase64: JSON 형식의 사용자 데이터로, Base64로 인코딩됩니다.
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64로 생성된 HMAC-SHA256 해시입니다.
- timestamp: 에포크 타임스탬프, **밀리초** 단위입니다. 미래 시점이거나 이틀 이상 과거일 수 없습니다.
- loginURL: 댓글 위젯이 사용자 로그인을 위해 표시할 수 있는 URL입니다.
- logoutURL: 댓글 위젯이 사용자 로그아웃을 위해 표시할 수 있는 URL입니다.
- loginCallback: 로그인 URL 대신 제공되는 경우, 로그인 버튼을 클릭할 때 댓글 위젯이 호출할 함수입니다.
- logoutCallback: 로그아웃 URL 대신 제공되는 경우, 로그아웃 버튼을 클릭할 때 댓글 위젯이 호출할 함수입니다.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO 클라이언트 코드'; isFunctional = false; code-example-end]

#### 사용자 객체

사용자(User) 객체는 다음 스키마를 포함합니다:
[inline-code-attrs-start title = '사용자 객체'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 필수. 최대 1k자. **/
    id: string;
    /** 필수. 최대 1k자. 참고: 고유해야 합니다. **/
    email: string;
    /** 필수. 최대 1k자. 참고: 사용자명은 이메일일 수 없습니다. 고유할 필요는 없습니다. **/
    username: string;
    /** 선택 사항. URL의 경우 최대 3k자. 기본값은 이메일 기반 gravatar입니다. 64 인코딩 이미지를 지원하며, 이 경우 제한은 50k자입니다. **/ 
    avatar?: string;
    /** 선택 사항. 기본값 false. **/
    optedInNotifications?: boolean;
    /** 선택 사항. 기본값 false. **/
    optedInSubscriptionNotifications?: boolean;
    /** 선택 사항. 최대 100자. 이 라벨은 이름 옆에 표시됩니다. 해당되는 경우 기본값은 Administrator/Moderator입니다. **/
    displayLabel?: string;
    /** 선택 사항. 최대 500자. 사용자명 대신 표시됩니다. **/
    displayName?: string;
    /** 선택 사항. 최대 2k자. 사용자 이름이 이 주소로 링크됩니다. **/
    websiteUrl?: string;
    /** 선택 사항. 사용자당 최대 100개 그룹. 그룹 id는 50자를 넘을 수 없습니다. **/
    groupIds?: string[];
    /** 선택 사항. 사용자를 관리자로 표시합니다. **/
    isAdmin?: boolean;
    /** 선택 사항. 사용자를 중재자로 표시합니다. **/
    isModerator?: boolean;
    /** 선택 사항, 기본값 true. 사용자 프로필에서 "activity" 탭을 활성화하려면 false로 설정합니다. **/
    isProfileActivityPrivate?: boolean;
    /** 선택 사항, 기본값 false. 프로필 댓글을 비활성화하려면 true로 설정합니다. **/
    isProfileCommentsPrivate?: boolean;
    /** 선택 사항, 기본값 false. 이 사용자에게 직접 메시지를 보내는 것을 비활성화하려면 true로 설정합니다. **/
    isProfileDMDisabled?: boolean;
    /** 사용자 배지에 대한 선택적 설정. **/
    badgeConfig?: {
        /** 할당할 전역 배지 ID 배열. 최대 30개 배지로 제한됩니다. 순서가 유지됩니다. **/
        badgeIds: string[];
        /** 현재 페이지(urlId)로 범위가 지정된 배지 ID 배열. 할당된 페이지에만 표시됩니다. **/
        pageBadgeIds?: string[];
        /** true인 경우 기존 표시된 배지를 대체합니다. 전역 및 페이지 범위 배지는 독립적으로 재정의됩니다. **/
        override?: boolean;
        /** true인 경우 테넌트 설정에서 배지 표시 속성을 업데이트합니다. **/
        update?: boolean;
    };
}
[inline-code-end]

#### 중재자 및 관리자

관리자 및 중재자의 경우 `SSOUser` 객체에 각각 `isAdmin` 또는 `isModerator` 플래그를 전달합니다.

#### 알림

알림을 활성화 또는 비활성화하려면 `optedInNotifications` 값을 각각 `true` 또는 `false`로 설정합니다. SSO 페이로드에 이 값이 포함된 상태로 사용자가 처음 페이지를 로드하면 알림 설정이 업데이트됩니다.

또한 사용자가 앱 내 알림뿐만 아니라 구독한 페이지의 활동에 대한 알림 이메일을 받도록 하려면 `optedInSubscriptionNotifications`를 `true`로 설정합니다.

#### VIP 사용자 및 특별 라벨

선택적인 "displayLabel" 필드를 사용하여 사용자 이름 옆에 특별 라벨을 표시할 수 있습니다.

#### 인증되지 않은 사용자

인증되지 않은 사용자를 나타내려면 userDataJSONBase64, verificationHash, timestamp를 채우지 않으면 됩니다. loginURL을 제공합니다.

이러한 사용자는 댓글을 작성할 수 없으며, 대신 로그인 메시지(설정에 따라 메시지, 링크 또는 버튼)가 표시됩니다.

#### 사용자 데이터 직렬화 및 해싱 직접 예제

자세한 내용은 예제로 <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">여기</a>(js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">여기</a>(java), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">여기</a>(php)에서 확인할 수 있습니다.

모든 통합이 복잡하고 어려운 과정이 될 수 있다는 것을 우리는 이해하고 있습니다. 담당자에게 문의하거나 <a href="https://fastcomments.com/auth/my-account/help" target="_blank">지원 페이지</a>를 이용하는 것을 주저하지 마세요.