Page Reactsでは、次の2点を決める必要があります:

- 使用するリアクション画像。
- 各リアクションに付ける短い `id`。

Optionally:

- 選択済み/未選択のリアクション用に別の画像を任意で定義することもできます。
- リアクションの上にマウスを移動したときに、リアクションしたユーザーの一覧を表示するかどうかを決められます。

[inline-code-attrs-start title = 'Page Reacts のコード例'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.fcConfigs = [{
        target: '#page-reacts-example',
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
    }];
</script>
[inline-code-end]

React、Angularなどのフロントエンドライブラリ向けの設定は同じです。

---