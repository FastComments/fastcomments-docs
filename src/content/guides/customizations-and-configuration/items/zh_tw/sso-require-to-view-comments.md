FastComments SSO(<a href="#sso">詳情請見此處</a>)讓您的使用者無需登入其他平台即可留言。

然而,僅憑這一點並無法保護您的留言串安全,因為預設情況下留言資料是公開資訊——任何能檢視該頁面的人都能看到留言。

透過更改一項設定,我們可以限制留言的讀取權限,僅允許管理員或有效的 SSO 使用者存取。

#### 無程式碼設定

當 SSO 已設定時,我們可以透過建立一條<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自訂規則</a>來防止檢視及互動留言串。

操作時,搜尋 SSO,您會找到這個選項:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='在自訂規則中啟用「需要 SSO 才能檢視留言」選項,並顯示安全層級選擇'; title='需要 SSO 才能檢視留言' app-screenshot-end]

啟用它並儲存自訂規則。

#### 僅保護特定網域或頁面

若要僅保護特定網域或頁面,我們只需在自訂規則中進行相應設定。

在自訂 UI 的頂部,我們會看到兩個輸入欄位:Domain 和 URL ID。

若要保護特定網域,請在「domain」欄位中輸入該網域。

若要保護特定頁面,請在「URL ID」欄位中輸入頁面網址。如果您與 FastComments 有自訂整合,也可以在此輸入某種類型的 ID 而非網址。

#### 安全層級

當要求 SSO 時,您需要決定是要求 Simple SSO 還是 Secure SSO。如果您要求 Simple SSO,則兩者皆可使用;但如果您要求 Secure SSO,則必須使用以您的 API 金鑰雜湊的 Secure SSO 負載來讀取內容,才能檢視。

當您選取「需要 SSO 才能檢視留言」時,安全層級選項就會出現,且僅在該選項開啟時生效。

<a href="#sso-simple">透過 Simple SSO 自動驗證留言並允許 Simple SSO 管理員</a>等設定是獨立的,無論此選項是否開啟皆會套用。

#### 閱讀以外的保護

啟用此選項後,除非使用者已透過 SSO 登入,否則該頁面或網域將無法被留言。

#### 注意事項

在您整合 SSO 之前建立留言的使用者,將無法看到自己的留言,除非他們透過您的 SSO 整合登入。