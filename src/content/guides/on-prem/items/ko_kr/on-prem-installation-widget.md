온프레미스용 프런트엔드 코드 스니펫과 라이브러리는 SaaS 제품과 동일합니다. 그러나 `apiHost`와 올바른 스크립트 경로를 지정해야 합니다:

[inline-code-attrs-start title = '온프레미스용 댓글 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... SSO 페이로드 등을 전달할 수도 있습니다.
    }];
</script>
[inline-code-end]

위 예시는 매우 단순한 예입니다. 또한 1st-party React, Angular, Vue, Svelte 등 라이브러리를 사용할 수도 있습니다.