对于页面反应 (Page Reacts)，我们需要决定两件事：

- 要使用哪些反应图片。
- 为每个反应指定一个简短的 `id`。

可选：

- 您也可以为已选中/未选中的反应定义单独的可选图片。
- 您可以决定在鼠标悬停在某个反应上时是否显示做出该反应的用户列表。 

[inline-code-attrs-start title = '页面反应代码示例'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

对于 React、Angular 等前端库，配置相同。