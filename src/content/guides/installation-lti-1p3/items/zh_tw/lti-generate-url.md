#### 前往 LTI 1.3 設定頁面

登入 FastComments 並前往 <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">您的 LTI 1.3 設定頁面</a>。

如果您的帳戶尚未擁有 LTI 存取權限，您會看到「LTI not enabled for this account」——請聯絡支援團隊以在您的方案中啟用此功能。

#### 選擇平台（選填）

在 **Generate a Dynamic Registration URL** 下方，使用 **Platform** 下拉選單告訴 FastComments 您要連接的 LMS：

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- 其他 LTI 1.3 平台

您也可以保留 **Auto-detect**。平台資訊會在註冊過程中從您 LMS 的 openid-configuration 讀取；下拉選單僅用於為產生的設定預先填入顯示標籤。

#### 產生 URL

點擊 **Generate URL**。FastComments 會建立一個一次性註冊權杖，並顯示一個如下所示的 URL：

`https://fastcomments.com/lti/v1p3/register/<long-token>`

請複製它。此 URL：

- 為**單次使用**——一旦您的 LMS 成功呼叫它，該權杖即被消耗。
- 若未使用，將於 **30 分鐘**後失效。
- 應妥善保密——任何擁有此 URL 的人都可以在這 30 分鐘內針對您的租戶註冊工具。

#### 現有設定

一旦註冊成功完成，新設定將顯示在同一頁面的 **Existing Configurations** 表格中，包含其 Platform、Issuer、Client ID 和 Status。如果需要取消註冊，您可以從此表格中刪除設定。