[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

`urlId` はコメントが紐付けられるページまたは記事の ID であることについては既に説明しました。

また、おさらいとして、`urlId` が定義されていない場合は現在のページの URL がデフォルトとして使用されます。

では、ページの再読み込みなしにコメントが紐付けられるページやコンテンツが動的に変化する SPA(シングルページアプリケーション)の場合はどうでしょうか?

#### Angular、React、Vue など

Angular や React などの当社のライブラリでは、ウィジェットに渡される `urlId` プロパティを更新するだけで、コメントウィジェットが更新されます。例えば React アプリでの動作は、<a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">こちら</a>で確認できます。

#### VanillaJS

VanillaJS ライブラリを使用している場合、Angular や React のようなデータバインディングや状態伝播を処理するフレームワークがないため、少し複雑になります。

VanillaJS ウィジェットをインスタンス化すると、ウィジェットを更新するために呼び出せる関数がいくつか返されます。

以下は、ページハッシュを変更してコメントウィジェットを更新する実際に動作する例です:

[inline-code-attrs-start title = 'ページハッシュ切り替えの例'; inline-code-attrs-end]
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

            config.url = locationString; // 通知が正しいページにリンクできるよう、url も更新します
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]