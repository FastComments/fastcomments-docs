Custom Code 탭의 **Footer** 섹션에 다음 코드를 붙여넣으세요:

[inline-code-attrs-start title = 'Typeflo.io 라이브 댓글 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Footer 섹션에 코드 붙여넣기</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Footer 섹션에 FastComments 코드 붙여넣기" />
</div>

코드를 붙여넣은 후 변경 사항을 적용하려면 **Save** 버튼을 클릭하세요.

참고: 이 코드는 Typeflo.io 블로그 게시물에서 FastComments 위젯을 동적으로 최적의 위치에 배치하는 로직을 포함합니다. 다른 코드 스니펫은 Typeflo.io의 레이아웃에서 제대로 작동하지 않을 수 있습니다.

가입 후 실제 FastComments 테넌트 ID로 'demo'를 교체해야 합니다. FastComments.com에 로그인한 상태라면 이미 교체되어 있을 것입니다.