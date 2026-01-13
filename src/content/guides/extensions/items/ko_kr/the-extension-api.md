`Extension`과 상호작용하는 것은 간단합니다. 호출되길 원하는 함수들에 대한 참조를 정의하기만 하면 됩니다.

앞의 예제에서 이어서, 각 댓글의 상단에 HTML을 추가한다고 가정해봅시다:

[inline-code-attrs-start title = '간단한 확장 - 계속'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

이와 같이 HTML을 반환하면, dom-diffing 알고리즘을 통해 UI에 병합됩니다.

#### 댓글을 수동으로 다시 렌더링하기

초기 페이지 로드를 기다린 다음 `reRenderComment`를 호출하여 댓글을 수동으로 다시 렌더링할 수 있습니다:

[inline-code-attrs-start title = '댓글 재렌더링'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // timeout not required, just an example.
    }
})();
[inline-code-end]

---