[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

초기 페이지 로드 후 다크 모드를 토글할 수 있는 사이트의 경우, 이는 조금 더 복잡합니다.

먼저, Comment 위젯 라이브러리의 현재 모든 버전(React, Vue)은 각 저장소에 다크 모드 토글 예제를 포함하고 있습니다.

VanillaJS 위젯의 경우에는 좀 더 작업이 필요합니다. 먼저, FastCommentsUI는 "destroy"와 "update" 함수를 가진 객체를 반환합니다.

댓글 위젯 구성을 업데이트할 때마다 update 함수를 단순히 호출하면 됩니다. 다음은 VanillaJS 위젯으로 다크 모드를 토글하는 완전한 작동 예제입니다.

[inline-code-attrs-start title = '다크 모드 토글 전체 예제'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---