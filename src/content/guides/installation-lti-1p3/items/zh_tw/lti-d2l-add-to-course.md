此頁面說明在管理員註冊工具並建立部署後，如何將 FastComments 新增至 Brightspace 課程。如果工具尚未註冊，請先參閱 D2L 註冊指南。

Brightspace 提供兩種內容撰寫體驗：**Classic Content** 與 **New Content Experience**（也稱為 **Lessons**）。兩者皆支援 FastComments，但選單路徑不同。以下各節在兩者有所差異時都會同時說明。

#### 定位 FastComments 工具

FastComments 工具會出現在課程內容編輯器中的兩個位置：

1. 活動選擇器，從模組/單元的 **Add Existing** 按鈕（舊版 Brightspace 標示為 **Add Existing Activities**）進入。在目前的 Brightspace 版本中 FastComments 會直接出現在選擇器中；舊版則會將它放在 **External Learning Tools** 子選單下。任一途徑都會將 FastComments 作為一個獨立主題加入。
2. HTML 編輯器內的 **Insert Stuff** 對話方塊底下的 **LTI Advantage**。這會透過 LTI 深度連結流程將 FastComments 內嵌到 HTML 主題中。

如果 FastComments 在兩個選擇器中都未出現，表示該部署尚未對包含此課程的組織單位啟用。請您的 Brightspace 管理員開啟 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments 工具 > **View Deployments**，開啟該部署，並在 **Org Units** 底下將課程所屬的組織單位（或其父組織單位）加入。

#### 在模組中將 FastComments 新增為主題

Classic Content:

1. 開啟課程，並在導覽列點選 **Content**。
2. 選擇應該放置討論的模組（或透過 **Add a module** 新增一個）。
3. 點選 **Add Existing**（舊版 Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. 在選擇器中點選 **FastComments**。Brightspace 會在模組中建立一個主題並將您導回內容檢視。
5. 點擊新建立的主題。使用內嵌標題編輯器將其重新命名為具有描述性的名稱，例如 `FastComments Discussion`。

New Content Experience (Lessons):

1. 開啟課程並點選 **Content**。
2. 開啟應放置討論的單元與課程單元（lesson）。
3. 點選 **Add** > **Existing Activity** 並選擇 **FastComments**（舊版 Brightspace：置於 **External Learning Tools** 底下）。
4. 該活動會被加入到 lesson 中。
5. 點擊活動標題以重新命名。

任一使用者（講師或學生）第一次開啟該主題時，FastComments 會為該資源連結初始化討論串。討論串與資源連結 ID 綁定，因此重新命名或移動主題不會改變載入的討論串。

#### 在 HTML 主題中內嵌 FastComments

當您希望留言出現在閱讀內容、影片或其他內容下方，並與該主題頁面同頁顯示（而非作為獨立主題）時，請使用此流程。

1. 在模組/lesson 中開啟或建立一個 HTML 主題。
2. 點選 **Edit HTML** 以開啟 Brightspace 的 HTML 編輯器。
3. 將游標放在希望顯示討論串的位置。
4. 點選 **Insert Stuff** 按鈕（編輯器工具列中的拼圖圖示）。
5. 在 Insert Stuff 對話方塊中捲動至 **LTI Advantage** 並點選 **FastComments**。
6. FastComments 會開啟一個深度連結選擇器。確認放置位置（預設選項適用於內容討論）；點選 **Insert** 或 **Continue**。
7. Brightspace 會返回 HTML 編輯器並顯示一個代表 LTI 啟動的佔位區塊。於主題上點選 **Save and Close**。

當主題載入時，Brightspace 會將該佔位區塊替換為一個透過 LTI 自動啟動 FastComments 的 iframe。學生會在同頁面內看到討論串。

單一 HTML 主題可以容納多個深度連結的 FastComments 內嵌。每個內嵌都會擁有自己的討論串，因為每個深度連結會產生不同的資源連結 ID。

#### 模組主題 vs 內嵌快速連結

當下列情況時，選擇「模組主題」方式：

- 討論是該模組步驟的主要活動。
- 您希望該主題出現在 Brightspace 的目錄（table of contents）、完成追蹤與 Class Progress 中。

當下列情況時，選擇「內嵌」方式：

- 留言應該出現在同一頁面中其他內容之下。
- 您不希望在目錄中有一個可被追蹤完成情況的獨立項目。

#### 可見性、草稿與發布條件

新的 FastComments 主題預設對學生可見。若要在設定時先隱藏它：

1. 在內容編輯器中，點選主題標題（Classic）或活動上的三點選單（New Content Experience）。
2. 設定狀態為 **Draft**（Classic）或將 **Visibility** 關閉（New Content Experience）。

草稿主題對學生不可見。講師與助教仍可看到，並會顯示「Draft」標籤。

若要將主題限制給特定群組或分組（section）：

1. 開啟該主題。
2. 點選主題標題選單 > **Edit Properties In-place**（Classic）或 **Edit** > **Restrictions**（New Content Experience）。
3. 在 **Release Conditions** 底下點選 **Create**。
4. 選擇 **Group enrollment** 或 **Section enrollment**，選取該群組/分組，並儲存。

發布條件會與 FastComments 自身的角色對應一同作用。無法看到該主題的學生不會取得 LTI 啟動權限。

#### 學生第一次啟動時看到的情況

當學生點選該主題（或載入包含內嵌的 HTML 主題）時：

1. Brightspace 會在背景執行 LTI 1.3 啟動。
2. FastComments 會接收到學生的姓名、電子郵件、頭像 URL 與 LMS 角色，並自動為其登入。使用者不會看到 FastComments 的登入提示。
3. 該資源連結的討論串會在 Brightspace 的 iframe 中呈現。

啟動時的角色對應：

- Brightspace 的 `Administrator` 會成為該討論串的 FastComments 管理者（admin），擁有完整的審核、刪除、封鎖與設定存取權限。
- Brightspace 的 `Instructor` 會成為 FastComments 的版主（moderator）（可置頂、隱藏、刪除、封鎖）。
- 其他所有角色（`Learner`、`TeachingAssistant` 等）將成為一般留言者。

留言會歸屬於學生的 Brightspace 帳戶。如果學生在 Brightspace 中編輯其名稱或頭像，下一次 LTI 啟動時會同步變更。

#### iframe 高度與調整大小

FastComments 在每次討論串渲染與內容變更（新留言、展開回覆）時，會發出 `org.imsglobal.lti.frameResize` 的 postMessage。Brightspace 會監聽此訊息並調整 iframe 高度，以避免討論串被裁切或出現內部捲軸。

如果 iframe 保持固定且高度過短：

- 確認課程是透過 HTTPS 載入。Brightspace 的 postMessage 監聽器會拒絕混合內容 (mixed-content) 的框架。
- 確認沒有瀏覽器外掛封鎖 postMessage 通道。
- 對於在 HTML 主題中的內嵌，外圍 HTML 不得將 iframe 包在固定高度的容器中。請移除父元素上的任何內嵌屬性例如 `style="height: ..."`。

#### Brightspace 特有注意事項

**工具未顯示於 Add Existing 選擇器。** 表示該部署尚未對此課程的組織單位啟用。管理員需要將組織單位（或其父層）新增至部署的 **Org Units** 清單。僅註冊工具本身並不足夠；部署範圍控制哪些課程可以看到該工具。

**啟動時出現 `deployment_id` 不匹配。** FastComments 會在註冊時以 TOFU 的方式鎖定第一個看到的 `deployment_id`。如果管理員刪除原始部署並建立新的部署，從新部署發出的啟動會因部署不匹配錯誤而被拒絕。解法是重新註冊 FastComments（產生新的註冊 URL 並重新執行動態註冊）；舊的設定紀錄會被取代。

**工具啟動但顯示「Invalid LTI launch」。** 表示課程位於與部署不同的租戶/組織結構中，或部署在註冊後被停用。請重新檢查 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 切換以及該部署的組織單位清單。

**FastComments 內缺少名稱與角色。** Brightspace 會在 LTI 啟動中提供 Names and Role Provisioning Services (NRPS) 聲明。如果課程是從舊的 LTI 1.1 連結升級而來，啟動可能缺少 `name` 與 `email` 聲明。請透過 **Add Existing** 重新新增 FastComments 主題（不要遷移舊連結），以便啟動使用 LTI 1.3。

**內嵌顯示登入畫面而非自動 SSO。** 表示該 HTML 主題是以指向 FastComments 的純 `<iframe>` 插入，而非透過 **Insert Stuff** > **LTI Advantage**。純 iframe 會跳過 LTI 啟動，使用者會被導向 FastComments 的公開頁面。請刪除該 iframe 並透過 Insert Stuff 流程重新插入。