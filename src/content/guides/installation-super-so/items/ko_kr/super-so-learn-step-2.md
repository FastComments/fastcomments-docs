---
다음 단계에서는 아래에 미리 만들어진 위젯 코드를 복사해야 합니다.

FastComments.com에 로그인한 상태라면 아래 코드 스니펫에 이미 계정 정보가 포함되어 있습니다. 복사합시다:

[inline-code-attrs-start title = 'Super.so FastComments 협업 채팅 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // 기존 인스턴스 정리
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // 기존 상단 바가 존재하면 정리
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // 새 상단 바 생성
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // FastComments Collab Chat 초기화
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // 현재 pathname 업데이트
            currentPathname = window.location.pathname;
        }

        // 초기 로드
        load();

        // 변경 사항을 500ms마다 확인
        setInterval(() => {
            // pathname이 변경되면 다시 로드
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // 위젯이 제거되면 다시 로드
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // 컨테이너가 비워지면 다시 로드
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

이제 `Body` 영역에 붙여넣으세요:

<div class="screenshot white-bg">
    <div class="title">붙여넣은 코드</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="붙여넣은 코드" />
</div>

If you see a "this is a demo message" after pasting the code:

- fastcomments.com 계정에 로그인했는지 확인하세요.
- 타사(3rd party) 쿠키가 활성화되어 있는지 확인하세요.
- 그런 다음 이 페이지를 새로고침하고 코드 스니펫을 다시 복사하세요. `tenantId`가 귀하의 테넌트 식별자로 채워져 있어야 합니다.

---