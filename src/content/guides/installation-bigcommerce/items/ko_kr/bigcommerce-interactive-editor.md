FastComments를 BigCommerce의 페이지 빌더를 통해 추가하는 것은 권장되지 않습니다. 그렇게 하면 원하는 각 페이지에 코드를 수동으로 추가해야 합니다.

그러나 이렇게 하려는 경우, 다음 코드 스니펫을 사용해야 합니다. BigCommerce의 특성상 다른 튜토리얼의 코드 스니펫은 작동하지 않습니다:

[inline-code-attrs-start title = 'BigCommerce 페이지 빌더 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]