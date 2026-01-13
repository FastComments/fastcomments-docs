我們的 [WordPress 外掛](https://wordpress.org/plugins/fastcomments/) 有一個強大的基於 UI 的匯入機制。安裝外掛後，它會引導你將你的 WordPress 安裝與 FastComments 連結，並將你現有的留言資料複製過來。

**這是在不需手動複製或下載任何東西的情況下完成的。**

在遷移期間，UI 會向你顯示遷移流程。大多數遷移只需幾分鐘。

此機制的設計目的是在遷移期間不會對你的 WordPress 安裝造成過大的負載。

### CloudFlare 與防火牆

為了讓自動化的 WordPress 設定能正常運作，我們必須對你的 WordPress 安裝進行呼叫。像 CloudFlare 這類的防火牆可能會阻擋我們並導致整合失敗。在這種情況下，[我們可以提供給你](https://fastcomments.com/auth/my-account/help) 一組需加入白名單的 IP 以供整合使用。

### 資料所有權

在我們的 WordPress 遷移情況下，任何新增或更新的留言資料都會在背後自動同步回你的 WordPress 安裝。這表示，雖然留言由 FastComments 本身提供以減輕你 WordPress 部署的負載，**我們也**會將它們儲存在你的資料庫作為備份。這也表示如果你希望切換離開 FastComments，你的資料已經被遷移且是最新的。