本頁說明在管理員已註冊該工具並建立部署後，如何將 FastComments 新增到 Brightspace 課程。如果工具尚未註冊，請先參閱 D2L 註冊指南。

<div class="screenshot white-bg">
    <div class="title">將 FastComments 以單元主題嵌入於 Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments 在 Brightspace 單元內執行，顯示分線討論與 @ 提及選擇器" />
</div>

Brightspace 提供兩種內容編輯體驗：**Classic Content（經典內容）** 與 **New Content Experience（新內容體驗，也稱 Lessons）**。兩者都支援 FastComments，但選單路徑不同。下列各節在差異處都會同時說明兩種情況。

#### 定位 FastComments 工具

FastComments 工具會出現在課程內容編輯器中的兩個位置：

1. 從模組/單元的 **Add Existing** 按鈕（舊版 Brightspace 標示為 **Add Existing Activities**）開啟的活動選擇器。於現行 Brightspace 版本中 FastComments 會直接顯示在選擇器內；舊版則會將它放在 **External Learning Tools** 子選單下。任何一條路徑都會將 FastComments 新增為單獨的主題。
2. HTML 編輯器內的 **Insert Stuff** 對話框中，位於 **LTI Advantage** 底下。這會透過 LTI 深度連結流程將 FastComments 內嵌在 HTML 主題中。

如果 FastComments 未出現在任一選擇器中，表示該部署未對承載課程的組織單位啟用。請您的 Brightspace 管理員開啟 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**，開啟部署並在 **Org Units** 底下新增該課程的組織單位（或其上層單位）。

#### 在模組中新增 FastComments 為主題

Classic Content：

1. 開啟課程並在導覽列點選 **Content**。
2. 選取應放置討論的模組（或透過 **Add a module** 建立一個）。
3. 點選 **Add Existing**（舊版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在選擇器中點選 **FastComments**。Brightspace 會在模組中建立一個主題，並返回內容檢視。
5. 點選新的主題。使用內聯標題編輯器將其重新命名為具描述性的名稱，例如 `FastComments Discussion`。

New Content Experience（Lessons）：

1. 開啟課程並點選 **Content**。
2. 開啟應放置討論的單元與課程（unit/lesson）。
3. 點選 **Add** > **Existing Activity** 並選擇 **FastComments**（舊版 Brightspace：在 **External Learning Tools** 底下）。
4. 活動會被新增到課程中。
5. 點選活動標題以重新命名。

任何使用者（教師或學生）第一次開啟該主題時，FastComments 會為該資源連結初始化討論串。討論串綁定到資源連結 ID（resource link ID），因此重新命名或移動主題不會改變所載入的討論串。

#### 在 HTML 主題中內嵌 FastComments

當您希望留言出現在同一個主題頁面內、置於閱讀內容、影片或其他內容下方（而非作為獨立主題）時，請使用此流程。

1. 在模組/課程中開啟或建立一個 HTML 主題。
2. 點選 **Edit HTML** 開啟 Brightspace 的 HTML 編輯器。
3. 將游標置於想要顯示留言串的位置。
4. 點選 **Insert Stuff** 按鈕（編輯器工具列中的拼圖圖示）。
5. 在 Insert Stuff 對話框中，捲動至 **LTI Advantage** 並點選 **FastComments**。
6. FastComments 會開啟一個深度連結選擇器。確認放置位置（預設選項適用於內容討論）；點選 **Insert** 或 **Continue**。
7. Brightspace 會返回 HTML 編輯器並放入一個代表 LTI 啟動的佔位區塊。於主題上點選 **Save and Close**。

當主題載入時，Brightspace 會以 iframe 替換該佔位區塊，並透過 LTI 自動啟動 FastComments。學生會在線內看到討論串。

單一 HTML 主題可包含多個深度連結的 FastComments 嵌入。每個嵌入都會產生自己的討論串，因為每個深度連結會產生不同的資源連結 ID。

#### 模組主題 vs 內嵌快速連結

選擇「模組主題」方法當：

- 該討論是模組中此步驟的主要活動。
- 您希望該主題出現在 Brightspace 的目錄、完成追蹤以及 Class Progress 中。

選擇「內嵌嵌入」方法當：

- 留言應該置於同一頁面的其他內容下方。
- 您不希望在目錄中出現可被追蹤完成情形的獨立項目。

#### 可見性、草稿與發布條件

新的 FastComments 主題預設對學生可見。若想在設定期間先將其隱藏：

1. 在內容編輯器中，點選主題標題（Classic）或活動上的三點選單（New Content Experience）。
2. 設定狀態為 **Draft**（Classic）或關閉 **Visibility**（New Content Experience）。

草稿主題對學生不可見。教師與助教仍能看到該主題，並會顯示「Draft」徽章。

若要將主題限制於特定群組或班級章節：

1. 開啟主題。
2. 點選主題標題選單 > **Edit Properties In-place**（Classic）或 **Edit** > **Restrictions**（New Content Experience）。
3. 在 **Release Conditions** 底下點選 **Create**。
4. 選擇 **Group enrollment** 或 **Section enrollment**，選取群組/章節，並儲存。

發布條件會與 FastComments 自身的角色對應一併作用。看不到主題的學生不會觸發 LTI 啟動。

#### 學生首次啟動時看到的內容

當學生點選主題（或載入含嵌入的 HTML 主題）時：

1. Brightspace 會在背景執行 LTI 1.3 啟動。
2. FastComments 會接收學生的姓名、電子郵件、頭像 URL 以及 LMS 角色，並自動替學生登入。學生不會看到 FastComments 的登入提示。
3. 該資源連結的討論串會在 Brightspace 的 iframe 中呈現。

啟動時的角色對應：

- Brightspace `Administrator` 會成為該討論串的 FastComments **admin**（完整的管理、刪除、封鎖與設定存取權限）。
- Brightspace `Instructor` 會成為 FastComments **moderator**（釘選、隱藏、刪除、封鎖）。
- 其他所有角色（`Learner`、`TeachingAssistant` 等）都會成為一般留言者。

留言會以學生的 Brightspace 帳號歸屬。若學生在 Brightspace 編輯他們的姓名或頭像，下一次 LTI 啟動時會同步變更。

#### Iframe 高度與調整大小

FastComments 在每次呈現討論串或內容變更時（新增留言、展開回覆等），都會發送 `org.imsglobal.lti.frameResize` 的 postMessage。Brightspace 會監聽該訊息並調整 iframe 高度，以避免討論串被裁切或出現內部捲軸。

如果 iframe 高度維持固定且過短：

- 確認課程是否透過 HTTPS 載入。Brightspace 的 postMessage 監聽器會拒絕混合內容的 iframe。
- 確認沒有瀏覽器擴充套件阻擋 postMessage 通道。
- 對於在 HTML 主題中的內嵌嵌入，周圍的 HTML 不應將 iframe 包在固定高度的容器中。請移除父元素上的任何 inline `style="height: ..."`。

#### Brightspace 特有的注意事項

**工具未出現在 Add Existing 選擇器中。** 表示該部署未對此課程的組織單位啟用。管理員需要將該組織單位（或上層單位）新增到部署的 **Org Units** 清單中。僅註冊工具本身並不足夠；部署決定哪些課程可以看到該工具。

**啟動時出現 `deployment_id` 不吻合。** FastComments 會以 TOFU 方式將註冊時看到的第一個 `deployment_id` 鎖定。如果管理員刪除了原始部署並建立新的部署，來自新部署的啟動會因部署不符而被拒絕。解決方法是重新註冊 FastComments（產生新的註冊 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>）並重新執行動態註冊）；舊的設定記錄將會被取代。

**工具能啟動但顯示「Invalid LTI launch」。** 可能是課程位於與部署覆蓋範圍不同的租戶/組織結構，或是在註冊後該部署被停用。請重新檢查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切換與部署的組織單位清單。

**在 FastComments 中缺少名稱與角色。** Brightspace 的 LTI 啟動會帶有 Names and Role Provisioning Services (NRPS) 的宣告。如果課程是從舊的 LTI 1.1 連結升級而來，啟動可能缺少 `name` 與 `email` 的宣告。請透過 **Add Existing** 重新加入 FastComments 主題（不要遷移舊連結），以便啟動使用 LTI 1.3。

**嵌入顯示登入畫面而非自動 SSO。** 表示該 HTML 主題是以一般 `<iframe>` 指向 FastComments 的方式插入，而非透過 **Insert Stuff** > **LTI Advantage**。一般 iframe 會跳過 LTI 啟動，使用者會被導向公開的 FastComments 頁面。請刪除該 iframe 並透過 Insert Stuff 流程重新插入。