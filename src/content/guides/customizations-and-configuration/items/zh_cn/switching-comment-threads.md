[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

我们已经讲过 `urlId` 是评论所绑定的页面或文章的 id。

另外，回顾一下，如果未定义，`urlId` 将默认为当前页面的 URL。

对于 SPA（Single-Page-Applications，单页应用），当评论所绑定的页面或内容在不重新加载页面的情况下动态改变时，该怎么办？

#### Angular、React、Vue 等

使用我们的库（例如 Angular 和 React），只需更新传递给组件的 `urlId` 属性即可使评论组件刷新。例如，你可以在 React 应用中看到实际效果，<a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">这里</a>。

#### VanillaJS

如果使用 VanillaJS 库，情况会稍微复杂一些，因为没有像 Angular 或 React 这样的框架来处理数据绑定或状态传播。

当你实例化 VanillaJS 小部件时，它会返回一些可用于更新小部件的函数。

下面是一个实际的示例，我们在其中更改页面哈希并更新评论小部件：

[inline-code-attrs-start title = '切换页面哈希示例'; inline-code-attrs-end]
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

            config.url = locationString; // 我们也更新 url，这样通知可以链接回正确的页面
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---