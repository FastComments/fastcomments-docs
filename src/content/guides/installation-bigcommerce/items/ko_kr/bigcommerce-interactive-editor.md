---
FastComments를 BigCommerce의 Page Builder를 통해 추가하는 것은 권장되지 않습니다. 그렇게 하면 코드를 원하는 각 페이지에 수동으로 추가해야 하기 때문입니다.

하지만 이렇게 하기를 원한다면, 다음 코드 스니펫을 사용해야 합니다. BigCommerce의 특성상 다른 튜토리얼의 코드 스니펫은 작동하지 않습니다:

[inline-code-attrs-start title = 'BigCommerce 페이지 빌더 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        function attemptLoad() {
            if (loaded) {
                return;
            }
            if (!window.FastCommentsUI) {
                return;
            }
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo"
            });
            loaded = true;
        }
        attemptLoad();
        const interval = setInterval(function () {
            attemptLoad();
            if (loaded) {
                clearInterval(interval);
            }
        }, 300);
    })();
</script>
[inline-code-end]

---