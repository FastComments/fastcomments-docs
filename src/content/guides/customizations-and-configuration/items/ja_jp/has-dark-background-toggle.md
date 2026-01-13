[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

ページの初回読み込み後にダークモードの切り替えを許可するサイトでは、これにはもう少し手間がかかります。

まず、Commentウィジェットライブラリのすべての現在のバージョン（React、Vue）には、それぞれのリポジトリにダークモード切り替えの例があります。

VanillaJSウィジェットについては、もう少し作業が必要です。まず、FastCommentsUIは "destroy" と "update" の関数を持つオブジェクトを返します。

コメントウィジェットの設定を更新したいたびに、単純にupdate関数を呼び出すことができます。以下は、VanillaJSウィジェットでダークモードを切り替える完全に動作する例です。

[inline-code-attrs-start title = 'ダークモード切り替えの完全な例'; inline-code-attrs-end]
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