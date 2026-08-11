FastComments SSO（<a href="#sso">此處詳情</a>）為您的使用者提供一種無需登入其他平台即可發表評論的方式。

然而，僅此並不能保護您的評論串，因為預設情況下，評論資料是公開資訊——任何能檢視頁面的人都能看到評論。

透過更改設定，我們可以限制只有管理員或有效的 SSO 使用者才能取得評論。

#### 無程式碼設定

當設定了 SSO 時，我們可以透過建立<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自訂規則</a>來防止檢視和互動我們的評論串。

執行此操作時，搜尋 SSO，您會找到此選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='在自訂規則中啟用「需要 SSO 以檢視評論」選項，並可選擇安全等級'; title='需要 SSO 以檢視評論' app-screenshot-end]

啟用它並儲存自訂規則。

#### 僅保護特定網域或頁面

若只想保護特定的網域或頁面，我們只需設定自訂規則即可。

在自訂 UI 的頂部，我們會看到兩個輸入欄位：Domain（網域）和 URL ID。

若只想保護特定網域，請在「domain」欄位中輸入該網域。

若要保護特定頁面，請在「URL ID」欄位中輸入頁面 URL。如果您與 FastComments 有自訂整合，您也可以在此輸入某種 ID 代替 URL。

#### 安全等級

在要求 SSO 時，您需要決定是使用 Simple SSO（簡易 SSO）還是 Secure SSO（安全 SSO）。如果選擇 Simple SSO，則兩者皆可接受；但若選擇 Secure SSO，則必須使用以您的 API 金鑰雜湊的 Secure SSO 載荷來取得內容，才能檢視。

當您選取「Require SSO To View Comments」時，安全等級選項會出現。

#### 閱讀之外的保護

啟用此選項後，除非使用者透過 SSO 登入，否則將無法在該頁面或網域上發表評論。

#### 注意事項

在您整合 SSO 之前已發表評論的使用者，除非透過您的 SSO 整合登入，否則將無法看到自己的評論。