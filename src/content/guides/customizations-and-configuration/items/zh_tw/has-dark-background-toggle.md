[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

對於允許在初始頁面載入後切換深色模式的網站，這會比較複雜。

首先，所有現行版本的評論元件函式庫（React、Vue）在它們各自的程式庫中都有切換深色模式的範例。

對於 VanillaJS 元件，我們需要多做一些工作。首先，FastCommentsUI 會回傳一個含有 "destroy" 和 "update" 函式的物件。

我們可以在每次想要更新評論元件設定時簡單地呼叫 update 函式，如下所示。以下是一個使用 VanillaJS 元件切換
深色模式的完整可運作範例。

[inline-code-attrs-start title = '切換深色模式完整範例'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---