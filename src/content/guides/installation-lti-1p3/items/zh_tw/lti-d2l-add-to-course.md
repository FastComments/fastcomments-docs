本頁說明在管理員已註冊該工具並建立部署之後，如何將 FastComments 新增到 Brightspace 課程中。如果工具尚未註冊，請先參閱 D2L 的註冊指南。

<div class="screenshot white-bg">
    <div class="title">FastComments 嵌入為 Brightspace 單元主題</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments 在 Brightspace 單元內執行，顯示分線評論與 @ 提及選擇器" />
</div>

Brightspace 提供兩種內容撰寫體驗：**經典內容** 與 **新內容體驗（也稱為 Lessons）**。兩者都支援 FastComments，但選單路徑不同。下列每個章節在差異處都會同時覆蓋兩者。

#### 定位 FastComments 工具

FastComments 工具會出現在課程內容編輯器的兩處：

1. 從模組/單元的 **Add Existing** 按鈕（舊版 Brightspace 標示為 **Add Existing Activities**）進入的活動選擇器。FastComments 在目前的 Brightspace 版本會直接出現在選擇器中；舊版本則將其放在 **External Learning Tools** 子選單下。兩種路徑都會將 FastComments 新增為獨立的主題。
2. HTML 編輯器內的 **Insert Stuff** 對話方塊，位於 **LTI Advantage** 底下。這會透過 LTI 深度連結流程將 FastComments 內嵌於 HTML 主題。

如果 FastComments 未出現在這兩個選擇器中，表示該部署尚未對持有課程的組織單位啟用。請要求 Brightspace 管理員開啟 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**，打開該部署，並在 **Org Units** 底下將課程所屬的 org unit（或其上層 org unit）新增進去。

#### 在模組中新增 FastComments 為主題

經典內容：

1. 開啟課程，並在導覽列中點選 **Content**。
2. 選取應包含討論的模組（或透過 **Add a module** 建立新模組）。
3. 點選 **Add Existing**（舊版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在選擇器中點選 **FastComments**。Brightspace 會在模組中建立一個主題，並將你導回內容檢視。
5. 點選新的主題。使用內嵌標題編輯器將其重新命名為描述性的名稱，例如 `FastComments Discussion`。

新內容體驗（Lessons）：

1. 開啟課程並點選 **Content**。
2. 開啟應包含討論的單元與課程（unit/lesson）。
3. 點選 **Add** > **Existing Activity** 並選擇 **FastComments**（舊版 Brightspace：位於 **External Learning Tools** 底下）。
4. 該活動會被加入到課程中。
5. 點選活動標題以重新命名。

任何使用者（講師或學生）第一次開啟該主題時，FastComments 會為該資源連結初始化討論串。討論串綁定於 resource link ID，因此重新命名或移動主題並不會改變載入的討論串。

#### 在 HTML 主題中內嵌 FastComments

當你希望在同一主題頁面中於閱讀材料、影片或其他內容下方顯示評論（而不是建立獨立主題）時，使用此流程。

1. 在模組/課程中開啟或建立一個 HTML 主題。
2. 點選 **Edit HTML** 以開啟 Brightspace 的 HTML 編輯器。
3. 將游標放在要顯示評論串的位置。
4. 點選 **Insert Stuff** 按鈕（編輯器工具列中的拼圖圖示）。
5. 在 Insert Stuff 對話方塊中，捲動到 **LTI Advantage** 並點選 **FastComments**。
6. FastComments 會開啟深度連結選擇器。確認放置位置（預設選項適用於內容討論）；點選 **Insert** 或 **Continue**。
7. Brightspace 會帶你回到 HTML 編輯器，並顯示代表 LTI 啟動的占位區塊。對主題點選 **Save and Close**。

當主題載入時，Brightspace 會將占位區塊替換為一個 iframe，並透過 LTI 自動啟動 FastComments。學生會在內嵌位置看到討論串。

單一 HTML 主題可以包含多個透過深度連結的 FastComments 內嵌。每個內嵌都會擁有自己的討論串，因為每個深度連結會產生不同的 resource link ID。

#### 模組主題 vs 內嵌快速連結

當下列情況時，選擇「模組主題」方式：

- 討論是該模組步驟中的主要活動。
- 你希望該主題出現在 Brightspace 的目錄、完成追蹤與 Class Progress 中。

當下列情況時，選擇「內嵌嵌入」方式：

- 評論應顯示在同一頁面其他內容的下方。
- 你不希望在目錄中出現一個可被追蹤完成狀態的獨立項目。

#### 可見性、草稿與發布條件

新建立的 FastComments 主題預設對學生可見。若要在設定期間隱藏它：

1. 在內容編輯器中，點選主題標題（經典內容）或活動上的三點選單（新內容體驗）。
2. 將狀態設為 **Draft**（經典內容）或將 **Visibility** 關閉（新內容體驗）。

草稿主題對學生不可見。講師與助教仍可看到並帶有「Draft」徽章。

若要將主題限制於特定群組或分班：

1. 開啟該主題。
2. 點選主題標題選單 > **Edit Properties In-place**（經典內容）或 **Edit** > **Restrictions**（新內容體驗）。
3. 在 **Release Conditions** 底下點選 **Create**。
4. 選擇 **Group enrollment** 或 **Section enrollment**，選取群組/分班，然後儲存。

發布條件會與 FastComments 自身的角色對應一起作用。無法看到該主題的學生將不會得到 LTI 啟動。

#### 學生首次啟動時看到的內容

當學生點選該主題（或載入有內嵌的 HTML 主題）：

1. Brightspace 會在背景執行 LTI 1.3 啟動。
2. FastComments 會接收學生的姓名、電子郵件、頭像 URL 與 LMS 角色，並自動為其登入。沒有 FastComments 的登入提示。
3. 該資源連結的評論串會在 Brightspace 的 iframe 內呈現。

啟動時的角色對應：

- Brightspace 的 `Administrator` 會成為該討論串的 FastComments 管理員（admin）（具完整的版務、刪除、禁止與設定存取權）。
- Brightspace 的 `Instructor` 會成為 FastComments 的版主（moderator）（置頂、隱藏、刪除、禁止）。
- 其他所有角色（如 `Learner`、`TeachingAssistant` 等）皆為一般評論者。

評論會歸屬於學生的 Brightspace 帳號。若學生在 Brightspace 中編輯其姓名或頭像，下一次 LTI 啟動時會同步該變更。

#### Iframe 高度與調整大小

FastComments 在每次討論串渲染及內容變更（新評論、展開回覆）時，會發出 `org.imsglobal.lti.frameResize` 的 postMessage。Brightspace 會監聽此訊息並調整 iframe 高度，避免討論串被裁切或顯示內部捲軸。

如果 iframe 維持在固定且過短的高度：

- 確認課程是否以 HTTPS 載入。Brightspace 的 postMessage 監聽器會拒絕混合內容的框架。
- 確認沒有瀏覽器外掛阻擋 postMessage 通道。
- 對於 HTML 主題中的內嵌，外層 HTML 不可將 iframe 包在固定高度的容器中。從父元素移除任何 inline `style="height: ..."`。

#### Brightspace 特有的注意事項

**工具未顯示在 Add Existing 選擇器中。** 表示該部署尚未對此課程所屬的 org unit 啟用。管理員需要將 org unit（或其上層）新增到部署的 **Org Units** 清單中。僅註冊工具本身並不足夠；部署決定哪些課程能看到該工具。

**在啟動時出現 `deployment_id` 不相符。** FastComments 會在註冊時以 TOFU 方式鎖定（pin）它看到的第一個 `deployment_id`。若管理員刪除原本的部署並建立新的部署，來自新部署的啟動會因部署不相符錯誤而被拒絕。修正方式是重新註冊 FastComments（產生新的註冊 URL 並再次執行 Dynamic Registration）；舊的設定紀錄會被取代。

**工具啟動但顯示「Invalid LTI launch」。** 表示該課程在與部署覆蓋範圍不同的租戶/組織結構中，或該部署在註冊後被停用。請重新檢查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切換與部署的 org unit 清單。

**FastComments 內缺少名稱與角色。** Brightspace 的 LTI 啟動會帶有 Names and Role Provisioning Services (NRPS) 的宣告。如果某課程是從舊的 LTI 1.1 連結升級而來，啟動可能缺少 `name` 與 `email` 宣告。請透過 **Add Existing** 重新新增 FastComments 主題（不要遷移舊連結），以確保啟動使用的是 LTI 1.3。

**內嵌顯示登入畫面而非自動 SSO。** 表示該 HTML 主題是以一般的 `<iframe>` 指向 FastComments 插入，而非透過 **Insert Stuff** > **LTI Advantage**。一般 iframe 會跳過 LTI 啟動，使用者會被導向公開的 FastComments 頁面。請刪除該 iframe，並透過 Insert Stuff 流程重新插入。