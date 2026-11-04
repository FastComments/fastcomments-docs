當讓使用者在網站上新增內容，然後在各種不同類型的裝置上呈現該內容時，安全性涉及多個層面。

### 防止格式濫用

使用者可能會透過濫用文字格式，撰寫刻意在視覺上造成干擾的內容，從而降低討論的價值。

FastComments 採取了多項措施來防止格式方面的濫用：

- 大量重複的連續換行會被摺疊。
- 我們不呈現標題（它們會變成一般文字）。
- 我們不允許 CSS 或自訂顏色。

### 防止漏洞攻擊

在呈現 HTML 的系統中可能會產生漏洞攻擊。FastComments 採取了多項措施來防止這種情況：

- 我們僅允許明確定義的一組 HTML 標籤。
- 我們僅允許明確定義的一組 HTML 標籤屬性。
- 我們對所有輸入進行淨化與消毒。
  - 這是透過 [DOMPurify](https://www.npmjs.com/package/dompurify) 和 [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) 函式庫完成的。
  - 我們選擇這些函式庫是因為它們經過充分測試（每週下載量分別超過 400 萬和 100 萬次）。

這意味著使用者無法執行諸如撰寫 `<script>` 或 `<style>` 標籤，或嘗試在圖片或其他內容中加入 `onload=alert()` 之類的腳本。

我們允許的 HTML 標籤如下：

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

預設情況下不允許使用 `<iframe>` 標籤。如果您開啟「允許媒體嵌入」，iframe 也會被允許，但僅限於其來源是內建的可信任提供者清單（例如 YouTube、Vimeo、SoundCloud 和 Spotify）或您明確新增的主機名稱。來自任何其他來源的 iframe 都會被移除。