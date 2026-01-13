[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

我們已說明 `urlId` 是評論所綁定的頁面或文章識別碼。

再說一次，如果未定義，`urlId` 將預設為當前頁面的 URL。

那麼對於 SPA（單頁應用程式），當評論所綁定的頁面或內容在沒有重新載入整頁的情況下動態變更時該怎麼辦？

#### Angular、React、Vue 等

使用我們的函式庫（例如 Angular 與 React），只要更新傳入 widget 的 `urlId` 屬性，評論元件就會重新整理。你可以在 React 範例中看到這個行為，例如 <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">這裡</a>。

#### VanillaJS

如果你使用 VanillaJS 函式庫，情況會比較複雜，因為沒有像 Angular 或 React 這類的框架來處理資料綁定或狀態傳播。

當你實例化 VanillaJS widget 時，它會回傳一些函式，可藉由呼叫這些函式來更新元件。

下面是一個實用範例，示範如何更改頁面 hash 並更新評論元件：

[inline-code-attrs-start title = '切換頁面 Hash 範例'; inline-code-attrs-end]
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