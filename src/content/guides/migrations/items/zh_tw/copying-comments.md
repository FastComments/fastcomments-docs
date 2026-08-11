如果需要搬移資料，FastComments 提供一個自行服務的工具，用於在頁面與文章之間搬移評論。

以下是評論複製頁面的表單樣子：

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='複製評論表單，包含「來源 URL ID」欄位以及「目標 URL ID」與「URL」欄位'; title='The Copy Comment Form' app-screenshot-end]

### 填寫「來源」欄位

要決定從哪裡搬移評論，我們只需要知道來源的 `URL ID`。

如果在評論小工具設定中未傳遞 `urlId` 的值，則會使用頁面 URL 的「乾淨」版本。

您可以透過匯出評論來查看每則評論的 `URL ID` 值。

### 填寫「目標」欄位

要決定將評論搬移到哪裡，我們需要知道目標的 `URL ID` 與 `URL`。

`URL ID` 會是評論所屬的儲存桶。`URL` 欄位則用於讓您能從電子郵件或審核工具直接導向該評論。

#### WordPress

如果您使用 WordPress，則可以在遷移工具的「目標/來源」`URL ID` 欄位中直接輸入文章 ID，而非 URL。

---