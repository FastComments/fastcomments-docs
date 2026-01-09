---
最小的擴充功能範例為：

[inline-code-attrs-start title = '一個簡單的擴充功能'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

為了本範例，將此儲存為 `my-extension.js`，並讓它可透過 `https://example.com/my-extension.min.js` 存取。

這個擴充功能本身不會執行任何動作，只有在載入時會取回由核心留言函式庫所建立的擴充物件。

此 `Extension` 物件為單例，且不會與其他擴充功能共用。

接下來，要載入我們的擴充功能，必須告知留言小工具。例如：

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

關於功能範例，請參閱下一節。
---