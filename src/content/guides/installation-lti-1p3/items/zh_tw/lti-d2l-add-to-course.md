本頁說明在管理員註冊工具並建立部署後，如何將 FastComments 新增至 Brightspace 課程。如果尚未註冊工具，請先參閱 D2L 註冊指南。

<div class="screenshot white-bg">
    <div class="title">在 Brightspace 單元主題中嵌入的 FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments 在 Brightspace 單元內執行，顯示線程式留言與 @ 提及選擇器" />
</div>

Brightspace 提供兩種內容撰寫體驗：**Classic Content** 與 **New Content Experience**（也稱為 **Lessons**）。兩者都支援 FastComments，但選單路徑不同。以下每個章節在差異處都會同時涵蓋兩者。

#### Locate the FastComments Tool

FastComments 工具會出現在課程內容編輯器內的兩個位置：

1. 活動挑選器，可從模組/單元的 **Add Existing** 按鈕進入（舊版 Brightspace 標示為 **Add Existing Activities**）。在目前的 Brightspace 版本中 FastComments 會直接出現在挑選器中；舊版本則會將它置於 **External Learning Tools** 子選單下。任一路徑都會將 FastComments 新增為獨立主題。
2. HTML 編輯器內的 **Insert Stuff** 對話框下的 **LTI Advantage**。這會透過 LTI 深度連結流程將 FastComments 內嵌在 HTML 主題中。

如果 FastComments 在任一挑選器中都未出現，表示部署尚未對包含該課程的組織單元啟用。請要求您的 Brightspace 管理員開啟 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**，打開該部署，並在 **Org Units** 下新增課程所屬的組織單元（或其上層組織單元）。

#### Add FastComments as a Topic in a Module

Classic Content:

1. 開啟課程並在導覽列點選 **Content**。
2. 選取應包含討論的模組（或透過 **Add a module** 建立一個）。
3. 點選 **Add Existing**（舊版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在挑選器中，點選 **FastComments**。Brightspace 會在模組中建立一個主題並帶您回到內容檢視。
5. 點選新的主題。使用內嵌標題編輯器將其重新命名為具描述性的名稱，例如 `FastComments Discussion`。

New Content Experience (Lessons):

1. 開啟課程並點選 **Content**。
2. 開啟應包含討論的單元與 lesson。
3. 點選 **Add** > **Existing Activity** 並選擇 **FastComments**（舊版 Brightspace：位於 **External Learning Tools** 之下）。
4. 該活動會被加入到 lesson 中。
5. 點選活動標題以重新命名。

任何使用者（講師或學生）第一次打開該主題時，FastComments 會為該資源連結初始化留言串。留言串綁定到資源連結 ID，因此重新命名或移動主題不會改變載入的留言串。

#### Embed FastComments Inline in an HTML Topic

當您想要讓留言出現在同一主題頁面內的一篇閱讀、影片或其他內容下方，而不是作為獨立主題時，請使用此流程。

1. 在模組/lesson 中開啟或建立一個 HTML 主題。
2. 點選 **Edit HTML** 以開啟 Brightspace 的 HTML 編輯器。
3. 將游標放在留言串應出現的位置。
4. 點選 **Insert Stuff** 按鈕（編輯器工具列中的拼圖圖示）。
5. 在 Insert Stuff 對話框中，滾動到 **LTI Advantage** 並點選 **FastComments**。
6. FastComments 會開啟深度連結挑選器。確認放置位置（預設選項適用於內容討論）；點選 **Insert** 或 **Continue**。
7. Brightspace 會帶您回到 HTML 編輯器，畫面會顯示代表 LTI 啟動的占位區塊。點選主題上的 **Save and Close**。

當主題載入時，Brightspace 會將占位區塊替換為一個自動透過 LTI 啟動 FastComments 的 iframe。學生會在行內看到討論串。

單一 HTML 主題可以包含多個深度連結的 FastComments 嵌入。每個嵌入都會獲得自己的留言串，因為每個深度連結會產生不同的資源連結 ID。

#### Module Topic vs Inline Quicklink

何時選擇 **module topic** 方式：

- 討論是該模組步驟的主要活動。
- 您希望該主題出現在 Brightspace 的目錄、完成追蹤和 Class Progress 中。

何時選擇 **inline embed** 方式：

- 留言應該位於同一頁面上的其他內容下方。
- 您不希望在目錄中有一個單獨可追蹤完成狀態的項目。

#### Visibility, Draft, and Release Conditions

新的 FastComments 主題預設對學生可見。若要在設定期間將其隱藏：

1. 在內容編輯器中，點選主題標題（Classic）或活動上的三點選單（New Content Experience）。
2. 將狀態設為 **Draft**（Classic）或關閉 **Visibility**（New Content Experience）。

草稿主題對學生不可見。講師和助教仍會看見帶有「Draft」徽章的主題。

要將主題限制為特定群組或班級部分：

1. 開啟該主題。
2. 點選主題標題選單 > **Edit Properties In-place**（Classic）或 **Edit** > **Restrictions**（New Content Experience）。
3. 在 **Release Conditions** 下，點選 **Create**。
4. 選擇 **Group enrollment** 或 **Section enrollment**，選取群組/班級部分，然後儲存。

發布條件會與 FastComments 自身的角色對應共同生效。無法看到該主題的學生不會取得 LTI 啟動。

#### What Students See on First Launch

當學生點選主題（或載入含嵌入的 HTML 主題）時：

1. Brightspace 背後會執行 LTI 1.3 啟動。
2. FastComments 會接收學生的名稱、電子郵件、頭像 URL 與 LMS 角色，並自動為其登入。使用者不會看到 FastComments 的登入提示。
3. 該資源連結的留言串會在 Brightspace iframe 內呈現。

啟動時的角色對應：

- Brightspace 的 `Administrator` 會成為該留言串的 FastComments 管理員（admin），具備完整的版主權限、刪除、封鎖與設定存取。
- Brightspace 的 `Instructor` 會成為 FastComments 的 **moderator**（可釘選、隱藏、刪除、封鎖）。
- 其他所有角色（`Learner`、`TeachingAssistant` 等）會成為一般留言者。

留言會歸屬於學生的 Brightspace 帳號。如果學生在 Brightspace 編輯了他們的名稱或頭像，下一次 LTI 啟動會同步變更。

#### Lock Down Public Access (Recommended)

預設情況下，FastComments 的留言資料是公開可讀的。任何能猜到留言串的 URL 或 API 端點的人都能檢視其留言，即使在 Brightspace 之外也是如此。對於課程討論，您幾乎肯定會希望將檢視權限限制為僅限已註冊的學員。

打開您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> 並建立一個啟用 **Require SSO To View Comments** 的規則，然後將安全層級設為 **Secure SSO**，讓留言串只能透過簽名的 LTI 啟動載入。

請參閱 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) 取得完整操作說明，包括如何將規則限定到單一網域或頁面。

#### Iframe Height and Resize

FastComments 在每次留言串呈現及內容變更（新增留言、展開回覆）時，都會發出 `org.imsglobal.lti.frameResize` postMessage。Brightspace 會監聽此訊息並調整 iframe 高度，以避免留言串被裁切或出現內部捲軸。

若 iframe 持續固定為較短高度：

- 確認課程是透過 HTTPS 載入。Brightspace 的 postMessage 監聽器會拒絕混合內容的框架。
- 確認沒有瀏覽器擴充套件阻擋 postMessage 通道。
- 對於 HTML 主題中的內嵌嵌入，外層 HTML 不得將 iframe 包在固定高度的容器內。請移除父元素上的任何內聯 `style="height: ..."`。

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** 該部署尚未對此課程所屬的組織單元啟用。管理員需要將組織單元（或其上層）新增到部署的 **Org Units** 清單中。僅註冊工具並不足以讓課程看見該工具；部署決定哪些課程能見到該工具。

**`deployment_id` mismatch on launch.** FastComments 會將它第一次看到的註冊 `deployment_id` 視為可信（TOFU）。如果管理員刪除了原始部署並建立新部署，來自新部署的啟動會因部署不匹配而被拒絕。解法是重新註冊 FastComments（產生新的註冊 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>）並再次執行動態註冊）；舊的設定紀錄會被取代。

**Tool launches but shows "Invalid LTI launch".** 課程位於與部署覆蓋範圍不同的租戶/組織結構中，或是在註冊後部署被停用。請重新檢查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切換與該部署的組織單元清單。

**Names and roles missing inside FastComments.** Brightspace 的 LTI 啟動會包含 Names and Role Provisioning Services (NRPS) 聲明。如果某個課程是從較舊的 LTI 1.1 連結升級而來，啟動可能缺少 `name` 與 `email` 聲明。請透過 **Add Existing** 重新新增 FastComments 主題（不要遷移舊連結），以便啟動使用 LTI 1.3。

**Embed shows a login screen instead of auto-SSO.** 該 HTML 主題是以指向 FastComments 的純 `<iframe>` 插入，而非透過 **Insert Stuff** > **LTI Advantage**。純 iframe 會跳過 LTI 啟動，使用者會被帶到面向公眾的 FastComments 頁面。刪除該 iframe 並透過 Insert Stuff 流程重新插入。