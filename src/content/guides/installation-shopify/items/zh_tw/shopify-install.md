### 從 Shopify 應用商店安裝

1. 開啟 [FastComments 在 Shopify 應用商店的列表頁面](https://apps.shopify.com/fastcomments)。
2. 點選 **新增應用程式** 並在安裝流程中選擇您想要的方案。
3. 安裝完成後，Shopify 會將您重新導回 Shopify 內的 FastComments 管理介面。

這就是完整的安裝流程。您不需要將任何東西貼到主題檔案中。

### 為您設置的內容

安裝會自動執行您原本要手動完成的所有步驟：

- 會為您的商店建立一個 FastComments 租戶並將其連結到您的商店網域。
- 您商店的網址會被加入到該租戶的授權網域，讓評論載入時不會出現網域錯誤。
- 會寫入一個 `fastcomments.tenant_id` shop metafield，讓每個區塊都知道要對哪個租戶進行呈現。
- 預設會啟用您 Shopify 客戶的單一登入功能。
- 計費透過 Shopify 管理的價格機制處理。費用會顯示在您一般的 Shopify 帳單上。您可以在 Shopify 後台的 **設定 > 應用程式與銷售管道 > FastComments** 中升級、降級或取消。

如果您的商店在安裝此應用程式前已經是 FastComments 的客戶，安裝流程會重複使用既有的租戶，而不會建立新的租戶。

### 內嵌管理後台

當您從 Shopify 後台開啟 FastComments 應用程式時，會進入一個儀表板，裡面有可一鍵開啟完整 FastComments 後台的捷徑磚：

- **Dashboard**：帳戶設定、使用情況與訂閱詳細資訊。
- **Moderation Queue**：在整個商店中核准、拒絕及回覆評論。
- **Customize**：調整元件的顏色、字型、審核規則與設定。
- **Ratings & Reviews Helper**：如果您想使用「評論摘要」區塊，可在此設置星級評分與評論問題。

每個捷徑磚都會以一次性登入連結開啟 FastComments，因此您不需要另外註冊或登入。

### 下一步：將區塊新增到您的商店

打開 Shopify 主題編輯器（**線上商店 > 主題 > 自訂**），開啟您想要新增評論或評分的樣板，然後點選 **新增區塊**。FastComments 的區塊會顯示在 **應用程式** 底下。本指南的其餘部分將逐一說明每個區塊。