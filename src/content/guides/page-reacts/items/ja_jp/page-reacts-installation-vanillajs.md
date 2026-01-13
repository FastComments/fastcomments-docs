For Page Reactsでは、次の2つを決める必要があります:

- どのリアクション画像を使用するか。
- 各リアクションの名前にする短い `id`。

任意で:

- 選択済み／未選択のリアクション用に、別々の画像を定義することもできます。
- リアクションのいずれかにマウスを重ねたときに、リアクションしたユーザーの一覧を表示するかどうかを選択できます。

[inline-code-attrs-start title = 'Page Reacts のコード例'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

React、Angular などのフロントエンドライブラリ向けの設定も同じです。