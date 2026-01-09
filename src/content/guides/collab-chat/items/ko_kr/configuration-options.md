### 개요

FastComments Collab Chat은 표준 FastComments 댓글 위젯을 확장하여 기본 위젯의 모든 구성 옵션을 상속하면서 텍스트 주석에 특화된 몇 가지 옵션을 추가합니다.

### 필수 구성

#### tenantId

FastComments Tenant ID가 필요합니다. 이 값은 [FastComments 대시보드](https://fastcomments.com/auth/my-account/api-secret)에서 찾을 수 있습니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 특정 옵션

#### urlId

기본적으로 Collab Chat은 페이지 URL, 요소의 DOM 경로, 선택된 텍스트 범위를 기반으로 각 대화에 대한 고유 식별자를 생성합니다. 이 값을 사용자 지정 `urlId`로 재정의할 수 있습니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

이는 URL 구조가 변경될 수 있지만 동일한 대화를 유지하려는 경우나 여러 페이지에 걸쳐 주석을 공유하려는 경우에 유용합니다.

#### topBarTarget

사용자 수와 토론 수를 표시하는 상단 바의 표시를 제어합니다. 상단 바를 완전히 비활성화하려면 `null`로 설정하거나, 특정 위치에 렌더링하려면 DOM 요소를 제공하십시오.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 상단 바 비활성화
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// 상단 바를 사용자 지정 위치에 렌더링
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

페이지에 어두운 배경이 있을 때 다크 모드 스타일을 활성화합니다. 이 감지는 자동으로 이루어지지만 수동으로 덮어쓰는 것이 바람직할 수 있습니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

댓글 수가 변경될 때마다 호출되는 콜백 함수입니다. 배지나 페이지 제목과 같은 UI 요소를 업데이트하는 데 유용합니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 상속된 구성 옵션

Collab Chat은 표준 댓글 위젯을 확장하므로, 기본 FastComments 위젯의 모든 구성 옵션을 사용할 수 있습니다. 다음은 일반적으로 사용되는 옵션들입니다:

#### locale

위젯 UI의 언어를 설정합니다. FastComments는 수십 개의 언어를 지원합니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 스페인어
});
[inline-code-end]

#### readonly

모든 대화를 읽기 전용으로 만듭니다. 사용자는 기존 주석을 볼 수는 있지만 새로 만들거나 답글을 달 수는 없습니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

싱글 사인온을 사용하여 인증 시스템과 통합합니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO 구성
    }
});
[inline-code-end]

인증 옵션에 대한 전체 세부사항은 SSO 문서를 참조하세요.

#### maxReplyDepth

답글의 중첩 수준을 제어합니다. 기본적으로 Collab Chat은 이를 0으로 설정하여 모든 댓글이 플랫(중첩 답글 없음)합니다. 스레드 대화를 원하면 이 값을 변경할 수 있습니다.

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3단계 중첩 허용
});
[inline-code-end]

### 내부 구성

이 옵션들은 Collab Chat에 의해 자동으로 설정되며 재정의해서는 안 됩니다:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### 전체 예제

다음은 여러 구성 옵션을 함께 보여주는 예제입니다:

[inline-code-attrs-start title = "구성 예제"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // SSO 구성
    },
    maxReplyDepth: 1
});
[inline-code-end]

기본 위젯에서 상속되는 사용 가능한 모든 구성 옵션의 전체 목록은 메인 FastComments 구성 문서를 참조하세요.