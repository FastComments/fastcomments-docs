페이지 리액션에서는 두 가지를 결정해야 합니다:

- 어떤 리액션 이미지를 사용할지.
- 각 리액션의 이름으로 사용할 짧은 `id`.

선택적으로:

- 선택된/선택되지 않은 리액션을 위한 별도의 이미지를 정의할 수도 있습니다.
- 리액션 중 하나 위에 마우스를 올렸을 때 리액션한 사용자 목록을 표시할지 여부를 결정할 수 있습니다. 

[inline-code-attrs-start title = '페이지 리액션 코드 예제'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.FastCommentsUI(document.getElementById('page-reacts-example'), {
        tenantId: 'demo',
        pageReactConfig: {
            showUsers: true,
            reacts: [
                {id: 'droll', src: 'https://docs.fastcomments.com/images/emojis/droll.png'},
                {id: 'he', src: 'https://docs.fastcomments.com/images/emojis/heart-eyes.png'},
                {id: 'laugh', src: 'https://docs.fastcomments.com/images/emojis/laugh.png'},
                {id: 'puke', src: 'https://docs.fastcomments.com/images/emojis/puke.png', selectedSrc: 'https://docs.fastcomments.com/images/emojis/puke-bw.png' },
                {id: 'rofl', src: 'https://docs.fastcomments.com/images/emojis/rofl.png' },
            ]
        }
    });
</script>
[inline-code-end]

React, Angular 등의 프론트엔드 라이브러리에 대한 구성도 동일합니다.

---