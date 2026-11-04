本頁面說明在管理員註冊工具並建立部署後，如何將 FastComments 加入 Brightspace 課程。如果工具尚未註冊，請先參閱 D2L 註冊指南。

<div class="screenshot white-bg">
    <div class="title">FastComments 以單元主題形式嵌入 Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments 在 Brightspace 單元中執行，顯示討論串與 @ 提及選擇器" />
</div>

Brightspace 提供兩種內容撰寫體驗：**Classic Content** 與 **New Content Experience**（也稱為 **Lessons**）。兩者皆支援 FastComments，但選單路徑不同。以下各節在兩者有差異之處會分別說明。

#### 尋找 FastComments 工具

FastComments 工具會出現在課程內容編輯器的兩個位置：

1. 活動選擇器，可從模組/單元的 **Add Existing** 按鈕進入（在較舊的 Brightspace 版本中標示為 **Add Existing Activities**）。在目前的 Brightspace 版本中，FastComments 會直接顯示在選擇器中；較舊版本則將其放在 **External Learning Tools** 子選單下。無論哪種路徑，都會將 FastComments 新增為獨立主題。
2. HTML 編輯器內的 **Insert Stuff** 對話框，位於 **LTI Advantage** 之下。這會透過 LTI deep linking 流程將 FastComments 內嵌於 HTML 主題中。

如果 FastComments 未出現在任一選擇器中，表示該部署未對課程所屬的 org unit 啟用。請您的 Brightspace 管理員開啟 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments 工具 > **View Deployments**，開啟該部署，並在 **Org Units** 下新增課程的 org unit（或其上層 org unit）。

#### 將 FastComments 新增為模組中的主題

Classic Content：

1. 開啟課程並在導覽列點擊 **Content**。
2. 選擇要放置討論的模組（或透過 **Add a module** 建立一個）。
3. 點擊 **Add Existing**（較舊的 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在選擇器中點擊 **FastComments**。Brightspace 會在模組中建立一個主題，並將您帶回內容檢視畫面。
5. 點擊新主題。使用內嵌標題編輯器將其重新命名為具描述性的名稱，例如 `FastComments Discussion`。

New Content Experience (Lessons)：

1. 開啟課程並點擊 **Content**。
2. 開啟要放置討論的單元與課程。
3. 點擊 **Add** > **Existing Activity** 並選擇 **FastComments**（較舊的 Brightspace：位於 **External Learning Tools** 之下）。
4. 活動會被加入課程中。
5. 點擊活動標題以重新命名。

任何使用者（教師或學生）第一次開啟該主題時，FastComments 會為該 resource link 初始化討論串。討論串綁定於 resource link ID，因此重新命名或移動主題不會改變載入的討論串。

#### 在 HTML 主題中內嵌 FastComments

當您希望評論出現在同一主題頁面中的閱讀內容、影片或其他內容下方，而非以獨立主題呈現時，請使用此流程。

1. 在模組/課程中開啟或建立一個 HTML 主題。
2. 點擊 **Edit HTML** 以開啟 Brightspace HTML 編輯器。
3. 將游標放在評論串應出現的位置。
4. 點擊 **Insert Stuff** 按鈕（編輯器工具列上的拼圖圖示）。
5. 在 Insert Stuff 對話框中，捲動至 **LTI Advantage** 並點擊 **FastComments**。
6. FastComments 會開啟 deep linking 選擇器。確認放置方式（預設選項適用於內容討論）；點擊 **Insert** 或 **Continue**。
7. Brightspace 會帶您回到 HTML 編輯器，並顯示代表 LTI 啟動的佔位區塊。在主題上點擊 **Save and Close**。

當主題載入時，Brightspace 會將佔位區塊替換為透過 LTI 自動啟動 FastComments 的 iframe。學生會看到內嵌的討論串。

單一 HTML 主題可容納多個 deep-linked 的 FastComments 嵌入。每個嵌入都有各自的討論串，因為每個 deep link 都會產生不同的 resource link ID。

#### 模組主題與內嵌 Quicklink 的比較

在以下情況選擇 **模組主題** 方式：

- 討論是該模組步驟中的主要活動。
- 您希望主題出現在 Brightspace 的目錄、完成度追蹤與 Class Progress 中。

在以下情況選擇 **內嵌嵌入** 方式：

- 評論應位於同一頁面上其他內容的下方。
- 您不希望在目錄中出現一個可追蹤完成度的獨立項目。

#### 可見性、草稿與發布條件

新的 FastComments 主題預設對學生可見。若要在設定時隱藏它：

1. 在內容編輯器中，點擊主題標題（Classic）或活動上的三點選單（New Content Experience）。
2. 將狀態設為 **Draft**（Classic）或關閉 **Visibility**（New Content Experience）。

草稿主題對學生不可見。教師與助教仍會看到它們並帶有「Draft」標記。

若要將主題限制於特定群組或分組：

1. 開啟主題。
2. 點擊主題標題選單 > **Edit Properties In-place**（Classic）或 **Edit** > **Restrictions**（New Content Experience）。
3. 在 **Release Conditions** 下，點擊 **Create**。
4. 選擇 **Group enrollment** 或 **Section enrollment**，選取群組/分組，然後儲存。

發布條件會與 FastComments 自身的角色對應疊加。無法看到主題的學生不會進行 LTI 啟動。

#### 學生首次啟動時看到的內容

當學生點擊主題（或載入含嵌入的 HTML 主題）時：

1. Brightspace 在背景執行 LTI 1.3 啟動。
2. FastComments 接收學生的姓名、電子郵件、頭像 URL 與 LMS 角色，並自動為其登入。不會出現 FastComments 登入提示。
3. 該 resource link 的評論串會在 Brightspace 的 iframe 內呈現。

啟動時的角色對應：

- Brightspace `Administrator` 會成為該討論串的 FastComments **admin**（完整的審核、刪除、封鎖與設定權限）。
- Brightspace `Instructor` 會成為 FastComments **moderator**（置頂、隱藏、刪除、封鎖）。
- 所有其他角色（`Learner`、`TeachingAssistant` 等）則成為一般評論者。

評論會歸屬於學生的 Brightspace 帳號。如果學生在 Brightspace 中修改姓名或頭像，下次 LTI 啟動時會同步變更。

#### 鎖定公開存取（建議）

預設情況下，FastComments 的評論資料是公開可讀的。任何能猜出討論串 URL 或 API 端點的人都可以檢視其評論，即使是在 Brightspace 之外。對於課程討論，您幾乎肯定會希望將檢視權限限制為已註冊的學習者。

開啟您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget 自訂頁面</a>，建立一條啟用 **Require SSO To View Comments** 的規則，然後將安全層級設為 **Secure SSO**，如此討論串便只能透過簽署的 LTI 啟動載入。

完整操作說明（包括如何將規則範圍限定於單一網域或頁面）請參閱 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)。

#### iframe 高度與調整大小

FastComments 會在每次討論串呈現以及內容變更（新評論、展開回覆）時發出 `org.imsglobal.lti.frameResize` postMessage。Brightspace 會監聽此訊息並調整 iframe 高度，使討論串不會被裁切，也不會出現內部捲軸。

如果 iframe 保持在固定的較小高度：

- 確認課程是透過 HTTPS 載入。Brightspace 的 postMessage 監聽器會拒絕混合內容的框架。
- 確認沒有瀏覽器擴充功能阻擋 postMessage 通道。
- 對於 HTML 主題中的內嵌嵌入，周圍的 HTML 不得將 iframe 包在固定高度的容器中。請移除父元素上任何內嵌的 `style="height: ..."`。

#### Brightspace 特有的注意事項

**工具未顯示在 Add Existing 選擇器中。** 部署未對此課程的 org unit 啟用。管理員需要將該 org unit（或其上層單位）加入部署的 **Org Units** 清單。僅註冊工具是不夠的；部署決定了哪些課程能看到該工具。

**啟動時 `deployment_id` 不符。** FastComments 會 TOFU 固定其為某註冊看到的第一個 `deployment_id`。如果管理員刪除原始部署並建立新的部署，來自新部署的啟動會因部署不符錯誤而被拒絕。解決方法是重新註冊 FastComments（產生新的註冊 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">由此取得</a>）並再次執行 Dynamic Registration）；舊的設定記錄會被替換。

**工具可啟動但顯示「Invalid LTI launch」。** 課程所在的租戶/org 結構與部署涵蓋範圍不同，或部署在註冊後被停用。請重新檢查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 開關以及部署的 org unit 清單。

**FastComments 內缺少姓名與角色。** Brightspace 的 LTI 啟動會附帶 Names and Role Provisioning Services (NRPS) 宣告。如果課程是從較舊的 LTI 1.1 連結升級而來，啟動會缺少 `name` 與 `email` 宣告。請透過 **Add Existing** 重新新增 FastComments 主題（不要遷移舊連結），使啟動使用 LTI 1.3。

**嵌入顯示登入畫面而非自動 SSO。** HTML 主題是以指向 FastComments 的普通 `<iframe>` 插入，而非透過 **Insert Stuff** > **LTI Advantage**。普通 iframe 會跳過 LTI 啟動，將使用者帶到公開的 FastComments 頁面。請刪除該 iframe，並透過 Insert Stuff 流程重新插入。