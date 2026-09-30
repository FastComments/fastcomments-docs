如果您正將網站從 WordPress 移走且想在新網站上使用 FastComments，則不需要 WordPress 外掛。從 WordPress 匯出您的評論，然後在 FastComments 儀表板的[匯入頁面](https://fastcomments.com/auth/my-account/manage-data/import)上傳檔案。

我們支援兩種 WordPress 匯出格式。

### WordPress XML（推薦）

這是 WordPress 內建匯出工具產生的檔案，無需額外外掛。

1. 在您的 WordPress 後台，前往 `Tools -> Export`。2. 選取 `All content` 並點擊 `Download Export File`。3. 在 FastComments 的[匯入頁面](https://fastcomments.com/auth/my-account/manage-data/import)，選取 `WordPress (.xml)` 並上傳檔案。

每則評論都與其所屬文章的 URL 相關聯，該資訊已包含在檔案中。

匯入時會保留作者名稱、電子郵件與網站、日期、內容、回覆串接，以及評論是否已核准。評論者的頭像會從 Gravatar 取得。投票不屬於此格式。

### WordPress CSV

這是來自[WebToffee 的 WordPress Comments Import & Export 外掛](https://wordpress.org/plugins/comments-import-export-woocommerce/)的檔案。

1. 在您的 WordPress 後台安裝此外掛，並將評論匯出為 CSV。2. 將每個 `comment_post_ID` 值替換為文章的 URL。3. 在 FastComments 的[匯入頁面](https://fastcomments.com/auth/my-account/manage-data/import)，選取 `WordPress (.csv)` 並上傳檔案。

每則評論都與 `comment_post_ID` 欄位相關聯。WordPress 會在此欄位填入文章 ID，而您的新網站沒有 WordPress 文章 ID，因此第 2 步會將其改為 URL。

匯入時會保留作者名稱、電子郵件與網站、日期、內容、回覆串接，以及評論是否已核准。評論者的頭像會從 Gravatar 取得。它也會保留 WordPress 的垃圾訊息標記，以及 wpDiscuz 的讚與倒讚（若檔案中包含這些資訊）。

### 將評論對應到新頁面

如果您的新網站保留與 WordPress 相同的 URL，評論會自動顯示在相對應的頁面，無需額外設定。

如果網域變更，請在匯入後執行[域名遷移工具](/guide-migrations.html#migrating-domains)。若個別頁面的 URL 變更，您可以[遷移每個頁面](/guide-migrations.html#migrating-pages)從舊 URL 轉至新 URL。

若需大量頁面遷移，例如從評論小工具的[urlId](/guide-customizations-and-configuration.html#url-id)欄位值中移除網域，請[開啟支援票證](https://fastcomments.com/auth/my-account/help)，我們會為您處理。

### 切換前

您可以隨意多次執行匯入。重新匯入相同檔案[不會產生重複項](/guide-migrations.html#importing-data)，因此您可以先匯入一次測試新網站，然後在切換前再次匯入最新的評論。

若匯出檔案大於 1GB，請[聯絡支援](https://fastcomments.com/auth/my-account/help)。

若要將 FastComments 加入您的新網站，請參考[安裝指南](/guide-installation.html)。