我們的 [WordPress 外掛](https://wordpress.org/plugins/fastcomments/) 擁有強大的基於 UI 的匯入機制。安裝外掛後，
它會引導您將 WordPress 安裝與 FastComments 連結，並複製您現有的留言資料。

**整個過程無需手動複製或下載任何內容。**

遷移過程會在遷移期間透過 UI 向您顯示。大多數遷移只需幾分鐘。

此機制的設計目的是在遷移期間不會對您的 WordPress 安裝造成過多的負載。

如果您要將網站從 WordPress 遷出，您可以匯入 WordPress XML 或 CSV 匯出檔，而不使用外掛。請參閱
[將您的留言移至新網站](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)。

### CloudFlare 與防火牆

為了讓自動化的 WordPress 設定能夠運作，我們必須對您的 WordPress 安裝進行呼叫。
像 Cloudflare 這樣的防火牆可能會封鎖我們並導致整合失敗。在這種情況下，[我們可以為您提供](https://fastcomments.com/auth/my-account/help) 一組需要加入白名單的 IP 以進行整合。

### 資料所有權

在我們的 WordPress 遷移中，任何新的或更新的留言資料都會在幕後自動同步回您的 WordPress 安裝。
這意味著，雖然留言由 FastComments 本身提供服務以減輕 WordPress 部署的負載，
我們**也**會將它們儲存在您的資料庫中作為備份。這也意味著，如果您想改用其他服務而離開 FastComments，您的資料已經遷移並保持最新狀態。