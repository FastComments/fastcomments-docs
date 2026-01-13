## Super.so Notion 문서에 라이브 댓글 위젯 추가하기

Collab Chat 외에도 Notion 문서 하단에 전통적인 댓글 위젯을 추가할 수 있습니다. 이를 통해 독자들이 댓글을 남기고 문서 전체에 대해 토론할 수 있습니다.

### 설치 단계

다음 코드를 복사하여 Super.so 사이트 설정의 `Body` 섹션에 붙여넣으세요:

[inline-code-attrs-start title = 'Super.so FastComments 라이브 댓글 위젯'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // 기존 인스턴스 정리
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // 새 대상 생성
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // FastComments 초기화
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // 현재 pathname 업데이트
            currentPathname = window.location.pathname;
        }

        // 초기 로드
        load();

        // 변경 사항 확인(500ms마다)
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

            // 컨테이너가 비어있으면 다시 로드
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### 중요 참고 사항

- 댓글 위젯이 Notion 문서 하단에 표시됩니다
- 각 페이지는 URL 경로를 기반으로 고유한 댓글 스레드를 가집니다
- FastComments 계정에서 실제 tenant ID로 "demo"를 반드시 교체하세요
- 이 위젯은 Super.so의 동적 페이지 로딩을 자동으로 처리합니다

---