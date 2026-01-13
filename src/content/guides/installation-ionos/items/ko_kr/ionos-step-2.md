다음으로 FastComments 위젯 코드를 사이트에 추가하겠습니다. 이 코드는 제목이 `FastComments Goes Here`인 모든 폼을 찾아 FastComments로 교체합니다.

그럼 사이트 편집기 왼쪽 하단의 `Settings`로 이동해 보겠습니다:

<div class="screenshot white-bg">
    <div class="title">설정 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="설정 열기" />
</div>

`Custom Head Code` 섹션을 엽니다:

<div class="screenshot white-bg">
    <div class="title">사용자 헤드 코드 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="사용자 헤드 코드 열기" />
</div>

Ionos의 경우 FastComments 위젯 코드의 **특별한 버전**이 필요합니다. **다른 튜토리얼의 코드 스니펫은 작동하지 않습니다.**

이제 다음 코드를 복사하세요:

[inline-code-attrs-start title = 'Ionos FastComments 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // 전체 너비가 아닌 요소를 가져옵니다
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...그리고 아래와 같이 붙여넣습니다:

<div class="screenshot white-bg">
    <div class="title">붙여넣기 및 저장</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="붙여넣기 및 저장" />
</div>

---