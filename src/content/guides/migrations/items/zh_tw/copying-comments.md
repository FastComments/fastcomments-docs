若需要移動資料，FastComments 提供一個自助工具來在頁面和文章之間移動評論。

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### 填寫 "From" 欄位

要決定要從哪裡移動評論，我們只需要知道來源的 `URL ID`。

如果你在評論小工具設定中沒有傳入 `urlId` 的值，則這將是頁面 URL 的「乾淨」版本。

你可以透過匯出來查看評論的 `URL ID` 值。

### 填寫 "To" 欄位

要決定要將評論移動到哪裡，我們需要知道目標的 `URL ID` 與 `URL`。

`URL ID` 將是評論歸入的桶（bucket）。`URL` 欄位則用於讓你能從電子郵件和審核工具直接導向該評論。

#### WordPress

如果你使用的是 WordPress，例如在遷移工具中的 To/From `URL ID` 欄位，你會填入文章 ID，而不是 URL。