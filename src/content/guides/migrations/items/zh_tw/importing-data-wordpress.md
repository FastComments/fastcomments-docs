我們的 [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) 具備強大的 UI 為基礎的匯入機制。安裝外掛後，  
它會指引您將 WordPress 安裝與 FastComments 連結，並複製您現有的評論資料。

**此過程不需要手動複製或下載任何檔案。**

遷移過程會在 UI 中顯示給您。大多數遷移只需幾分鐘。

此機制設計為在遷移期間不會對您的 WordPress 安裝造成過度負載。

如果您要將網站從 WordPress 移走，您可以匯入 WordPress 的 XML 或 CSV 匯出檔案，而不是使用外掛。請參閱  
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)。

### CloudFlare & FireWalls

為了讓自動化的 WordPress 設定能正常運作，我們必須呼叫您的 WordPress 安裝。  
像 Cloudflare 這樣的防火牆可能會阻擋我們，導致整合失敗。在此情況下，[我們可以提供  
您](https://fastcomments.com/auth/my-account/help) 一組需要列入白名單的 IP 位址，以完成整合。

### Data Ownership

在我們的 WordPress 遷移情況下，任何新建或更新的評論資料都會自動同步回您的 WordPress 安裝，  
在幕後完成。這表示，雖然評論是由 FastComments 本身提供，以減輕您的 WordPress 部署負載，  
我們 **也** 會將它們儲存於您的資料庫作為備份。這同時意味著，如果您想要轉離 FastComments，您的資料已經  
完成遷移且保持最新。