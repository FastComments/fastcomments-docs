다음으로 FastComments.com 위젯 코드를 추가할 위치를 확인해야 합니다.

기본 `casper` 테마를 사용하는 경우, `82`번째 줄에 다음과 같은 섹션을 볼 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">비활성화된 댓글 섹션</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="비활성화된 댓글 섹션" />
</div>

다른 테마를 사용하는 경우에는 이를 보지 못할 수 있으며, 마지막 `</section>` 태그 다음에 이 코드를 추가해야 합니다:

[inline-code-attrs-start title = '섹션 예제'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

다음과 같이 준비되어 있어야 합니다:

<div class="screenshot white-bg">
    <div class="title">댓글 코드를 위한 템플릿 준비됨</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="댓글 코드를 위한 템플릿 준비됨" />
</div>

준비가 되면 FastComments.com 위젯 코드를 복사합니다:

[inline-code-attrs-start title = 'Ghost FastComments.com 댓글 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    id: '\{{ @member.uuid }}',
                    email: '\{{@member.email}}',
                    username: '\{{@member.name}}',
                    avatar: '\{{ @member.avatar_image }}',
                    optedInNotifications: true,
                    optedInSubscriptionNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        });
    })();
</script>
[inline-code-end]

...그리고 다음과 같아야 합니다:

<div class="screenshot white-bg">
    <div class="title">FastComments.com 댓글 코드 추가</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com 댓글 코드 추가" />
</div>

코딩이 완료되었습니다. 이제 테마를 다시 가져오기만 하면 됩니다!