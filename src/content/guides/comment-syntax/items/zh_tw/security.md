當允許使用者在網站上新增內容，
並在各種不同類型的裝置上呈現該內容時，安全性有多方面的考量。

### Preventing Formatting Abuse

有人可能會透過濫用文字格式撰寫故意造成視覺干擾
並降低討論的價值。

FastComments 採取了多項措施來防止格式上的濫用：

- 大量連續重複的換行會被折疊。
- 我們不會渲染標題（它們會變成普通文字）。
- 我們不允許 CSS 或自訂顏色。

### Preventing Exploits

在渲染 HTML 的系統中可能會產生可被利用的漏洞。FastComments 採取了若干措施來防止此類情形：

- 我們僅允許一組明確定義的 HTML 標籤。
- 我們僅允許一組明確定義的 HTML 標籤屬性。
- 我們會淨化並過濾所有輸入。
  - 這是透過 [DOMPurify](https://www.npmjs.com/package/dompurify) 與 [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) 套件完成的。
  - 我們選擇這些套件因為它們經過良好測試（每週下載量分別超過 4 百萬 與 1 百萬次）。

這代表使用者無法做像是撰寫 `<script>` 或 `<style>` 標籤，或試圖在影像或其他內容上加入 `onload=alert()` 類型的腳本。

The HTML tags we allow are as follows:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`

The `<iframe>` tag is not allowed by default. If you turn on 允許媒體嵌入, iframes are also permitted, but only when their source is one of a built-in list of trusted providers (such as YouTube, Vimeo, SoundCloud, and Spotify) or a hostname you have explicitly added. Iframes from any other source are removed.