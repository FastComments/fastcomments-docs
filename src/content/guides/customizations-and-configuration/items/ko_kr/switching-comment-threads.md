[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

우리는 `urlId`가 댓글이 연결되는 페이지 또는 게시물의 ID라는 것을 다뤘습니다.

또한 요약하면, 정의되지 않으면 `urlId`는 현재 페이지 URL로 기본값이 설정됩니다.

페이지나 콘텐츠가 전체 페이지 리로드 없이 동적으로 변경되는 SPA(싱글 페이지 애플리케이션)의 경우는 어떨까요?

#### Angular, React, Vue 등

Angular 및 React와 같은 라이브러리를 사용하는 경우, 위젯에 전달되는 `urlId` 속성을 단순히 업데이트하면 댓글 위젯이 새로고침됩니다. 예를 들어 React 앱에서 동작하는 모습을 <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">여기</a>에서 확인할 수 있습니다.

#### VanillaJS

VanillaJS 라이브러리를 사용하고 있다면, 데이터 바인딩이나 상태 전파를 처리해 주는 Angular나 React 같은 프레임워크가 없기 때문에 약간 더 복잡합니다.

VanillaJS 위젯을 인스턴스화하면, 이를 업데이트할 수 있는 몇 가지 함수들이 반환됩니다.

다음은 페이지 해시를 변경하고 댓글 위젯을 업데이트하는 작동 예제입니다:

[inline-code-attrs-start title = '페이지 해시 전환 예제'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---