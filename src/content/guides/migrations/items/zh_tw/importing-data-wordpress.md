我們的[WordPress 插件](https://wordpress.org/plugins/fastcomments/)具備強大的基於 UI 的匯入機制。安裝插件後，
它會指引您將 WordPress 安裝與 FastComments 連結，並複製您現有的評論資料。

**這是在不手動複製或下載任何內容的情況下完成的。**

在遷移過程中，遷移流程會透過 UI 向您顯示。大多數遷移只需幾分鐘。

此機制的設計旨在遷移期間不對您的 WordPress 安裝造成過度負載。

### CloudFlare 與 防火牆

為了讓自動化的 WordPress 設定能正常運作，我們必須對您的 WordPress 安裝發出請求。  
像 Cloudflare 這樣的防火牆可能會阻擋我們，導致整合失敗。在此情況下，[我們可以提供您](https://fastcomments.com/auth/my-account/help)一組需要列入白名單的 IP 位址，以完成整合。

### 資料所有權

在我們的 WordPress 遷移情況下，任何新建或更新的評論資料都會在幕後自動同步回您的 WordPress 安裝。  
這表示，雖然評論由 FastComments 本身提供，以減輕您的 WordPress 部署負載，我們**也**會將它們儲存於您的資料庫作為備份。這同時意味著，如果您想要轉離 FastComments，您的資料已經完成遷移且保持最新。

---