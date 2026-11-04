### 개요

FastComments Collab Chat은 표준 FastComments 댓글 위젯을 확장한 것이므로, 기본 위젯의 모든 설정 옵션을 상속하면서 텍스트 주석(Annotation)에 특화된 몇 가지 옵션을 추가합니다.

### 필수 설정

#### tenantId

FastComments Tenant ID가 필요합니다. [FastComments 대시보드](https://fastcomments.com/auth/my-account/api-secret)에서 확인할 수 있습니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 전용 옵션

#### urlId

기본적으로 Collab Chat은 페이지 URL, 요소의 DOM 경로, 선택된 텍스트 범위를 기반으로 각 대화에 대한 고유 식별자를 생성합니다. 사용자 지정 `urlId`로 이를 재정의할 수 있습니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

URL 구조가 변경될 수 있지만 동일한 대화를 유지하고 싶거나, 여러 페이지에서 주석을 공유하고 싶을 때 유용합니다.

#### topBarTarget

사용자 수와 토론 수를 표시하는 상단 바의 표시를 제어합니다. `null`로 설정하면 상단 바를 완전히 비활성화하고, DOM 요소를 제공하면 특정 위치에 렌더링합니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

페이지에 어두운 배경이 있을 때 다크 모드 스타일을 활성화합니다. 이 감지는 자동으로 이루어지지만, 필요에 따라 재정의할 수 있습니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

댓글 수가 변경될 때마다 호출되는 콜백 함수입니다. 배지나 페이지 제목 같은 UI 요소를 업데이트할 때 유용합니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 상속된 설정 옵션

Collab Chat은 표준 댓글 위젯을 확장한 것이므로, 기본 FastComments 위젯의 모든 설정 옵션을 사용할 수 있습니다. 자주 사용되는 옵션은 다음과 같습니다:

#### locale

위젯 UI의 언어를 설정합니다. FastComments는 수십 개의 언어를 지원합니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

모든 대화를 읽기 전용으로 만듭니다. 사용자는 기존 주석을 볼 수 있지만 새 주석을 만들거나 답장할 수 없습니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Single Sign-On을 사용하여 인증 시스템과 통합합니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

인증 옵션에 대한 전체 세부 사항은 SSO 문서를 참조하세요.

#### maxReplyDepth

답글이 몇 단계까지 중첩될 수 있는지 제어합니다. 기본적으로 Collab Chat은 이 값을 0으로 설정하여 모든 댓글이 평면적(중첩된 답글 없음)이 되도록 합니다. 스레드 형태의 대화를 원하면 이 값을 변경할 수 있습니다.

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### 내부 설정

이 옵션들은 Collab Chat에 의해 자동으로 설정되며 재정의해서는 안 됩니다:

`productId`는 Collab Chat을 위해 자동으로 `3`으로 설정됩니다. 채팅 창 기능을 제공하기 위해 `floating-chat` 확장이 자동으로 로드됩니다. 위젯은 모바일 기기(너비 768px 미만 화면)를 자동으로 감지하여 그에 맞게 UI를 조정합니다.

### 전체 예시

여러 설정 옵션을 함께 사용하는 예시입니다:

[inline-code-attrs-start title = "설정 예시"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

기본 위젯에서 상속된 사용 가능한 모든 설정 옵션의 전체 목록은 FastComments 기본 설정 문서를 참조하세요.