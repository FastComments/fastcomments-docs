FastComments SSO (<a href="#sso">詳細資訊</a>) 為您的使用者提供一種在不必登入其他平台的情況下發表評論的方式。

然而，僅此並不會保護您的評論串，因為預設情況下評論資料為公開資訊 - 任何能查看該頁面的人都可以看到評論。

#### 無需程式碼設定

當設定 SSO 後，我們可以透過建立一個 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自訂規則</a> 來防止檢視及互動評論串。

執行此操作時，搜尋 SSO，您會找到此選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

啟用它並儲存該自訂規則。

#### 只保護特定的網域或頁面

若只想保護特定的網域或頁面，我們只需在自訂規則中進行設定即可。

在自訂介面的上方，我們會看到兩個輸入欄位：Domain 和 URL ID。

若僅保護某個網域，請在 "domain" 欄位輸入該網域。

若要保護特定頁面，請在 "URL ID" 欄位輸入頁面 URL。如果您與 FastComments 有自訂整合，您也可以在此輸入某種類型的 ID，而非 URL。

#### 安全等級

當要求 SSO 時，您需要決定使用 Simple SSO 還是 Secure SSO。若要求 Simple SSO，兩者都允許；但若要求 Secure SSO，則必須使用以您的 API key 進行雜湊的 Secure SSO 載荷來擷取內容，才能檢視。

當您選擇 "Require SSO To View Comments" 時，會出現安全等級選項。

#### 超出閱讀的保護

啟用此選項會保護該頁面或網域，禁止未透過 SSO 登入的使用者發表評論。

#### 注意事項

在您啟用 SSO 整合之前建立評論的任何使用者，將無法看到那些評論，除非他們透過您的 SSO 整合登入。