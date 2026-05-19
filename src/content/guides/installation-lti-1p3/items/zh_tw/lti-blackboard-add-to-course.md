一旦管理員已將 FastComments 註冊為 LTI 1.3 Advantage 工具並核准機構政策後，教師即可透過標準的 Blackboard 放置點將其新增到課程中。實際步驟在 Ultra Course View 與 Original Course View 之間有所不同，兩者皆在下方說明。

#### Ultra Course View

Ultra Course View 自 2026 年起為 Blackboard Learn SaaS 的預設。

1. 打開課程並前往 **Course Content** 頁面。
2. 在大綱中將滑鼠懸停或點擊希望放置留言串的位置，然後點選紫色 **+**（新增內容）按鈕。
3. 選擇 **Content Market**。Content Market 面板會列出機構核准的所有 LTI 工具與 Building Block 放置項目。
4. 找到 **FastComments** 方塊並點擊它。Blackboard 會在你開啟 **+** 選單的位置建立一個內容項目。
5. 預設情況下，該項目會以「對學生可見」條目出現在大綱中，前提是教師個人預設的 **Hide from students** 為關閉。如果你的預設為 **Hidden**，該項目會被建立為隱藏，當你準備就緒時，可以在該項目的列上切換可見性選項。
6. 如需重新命名該項目，點擊大綱中的標題並輸入新的標籤。學生在大綱中看到的標題與 FastComments 留言串識別符是獨立的，因此隨時重新命名都是安全的。

如果看不到 **Content Market** 選項，表示機構已將該放置點隱藏。你也可以透過相同 **+** 選單中的 **More tools**，在 **LTI Tools** 群組下開啟相同的選擇器。

#### Original Course View

Original Course View 在 Learn SaaS 中仍受支援，並且在 Q4 2024 CU 發行線上的自我託管 Learn 9.1 站點仍為主要體驗。

1. 打開課程並進入一個 **Content Area**（例如課程選單中的預設 **Information** 或 **Content** 區域）。
2. 在頁面右上角的切換開關將 **Edit Mode** 開啟。
3. 在動作列中點擊 **Build Content**。
4. 在 **Learning Tools** 子選單下，點擊 **FastComments**。Learning Tools 子選單會在管理員註冊工具後，根據 LTI 1.3 工具放置項目填入。如果看不到，請參閱下方的常見問題區。
5. 在 **Create FastComments** 表單中，設定：
   - **Name**：學生在內容區看到的標籤。
   - **Description**：可選文字，顯示於嵌入的留言串上方。
   - **Permit Users to View this Content**：是/否 的可用性切換。
   - **Track Number of Views**：若想要 Blackboard 的每項目檢視統計則啟用。FastComments 會獨立執行自己的分析。
   - **Date and Time Restrictions**：可選的 **Display After** / **Display Until** 時間窗。
6. 提交。該工具會以可點擊的項目出現在內容區中。

#### 將其嵌入到項目或文件內

在兩種課程檢視中，教師都可以透過內容編輯器的 LTI Advantage 按鈕，將 FastComments 內嵌到項目、文件或任何富文字欄位的內文中。

Ultra Course View：

1. 建立或編輯一個 **Document**。
2. 在文件內容中想要留言串出現的位置點擊 **Add content**。
3. 在編輯器工具列中打開 **Insert content** 選單並點擊 **Content Market**（LTI Advantage / Deep Linking 的進入點）。
4. 選擇 **FastComments**。FastComments 回傳深度連結（deep-link）載荷，Blackboard 會在游標位置插入一個嵌入區塊到文件內文中。
5. 儲存文件。學生在滾動過該處時會看到留言串內嵌呈現。

Original Course View：

1. 編輯任何具有富文字內容的項目。
2. 在內容編輯器工具列中，點擊 **Add Content** 的加號圖示並選擇 **Content Market**（在較舊的 Q4 2024 CU 中標示為 **Add Content from External Tool**）。
3. 選擇 **FastComments**。編輯器會插入一個參考深度連結資源的佔位區塊。
4. 提交該項目。

每個深度連結嵌入會建立自己的 FastComments 留言串，因此在同一個項目中嵌入兩個 FastComments 區塊就會有兩個獨立的留言串。

#### 可見性、發佈條件與群組限制

FastComments 的內容項目在存取控制規則上與其他 Blackboard 內容項目行為一致。

- Ultra：在該列上點擊可見性選擇器（**Visible to students**、**Hidden from students**、**Conditional availability**）。條件可用性支援日期/時間窗、針對成績簿項目的表現規則，以及針對課程群組的成員規則。
- Original：打開該項目的功能選單並選擇 **Adaptive Release** 或 **Adaptive Release: Advanced**，以根據日期、成員、成績或審閱狀態限制該工具的存取。使用項目的 **Set Group Availability** 來限制至特定課程群組。

FastComments 會遵從 Blackboard 所設定的任何存取門檻。若 Blackboard 將該項目對某個學生隱藏，該學生就不會發生 LTI 啟動，且不會出現在管理者檢視中。

#### 成績簿行為

FastComments 不會透過 LTI Advantage 的 Assignment and Grade Services 回傳成績。FastComments 內容項目不會自動建立成績欄位。

若你的 Blackboard 租戶設定為無論評分元資料為何都會自動為每個新內容項目建立成績簿欄位，則仍會出現一個空的欄位。若要隱藏它：

- Ultra：打開 **Gradebook**，點擊欄位標題，選擇 **Edit**，並關閉 **Show to students** 以及 **Include in calculations**。若你的機構允許刪除未評分項目的欄位，也可使用 **Delete**。
- Original：打開 **Grade Center**，點擊該欄位的箭頭，選擇 **Hide from Users (on/off)**，並在 **Column Organization** 下視需要選擇 **Hide from Instructor View**。

#### 學生會看到的內容

當學生打開 FastComments 項目或滾動到嵌入區塊時：

1. Blackboard 會向 FastComments 發送 LTI 1.3 訊息啟動。學生會透過 SSO 使用其 Blackboard 身分（姓名、電子郵件、頭像、角色）登入，且不會看到登入表單。
2. 留言串在 iframe 中呈現。根據 FastComments 中設定的留言元件選項，將可使用主題式留言、回覆、提及與表情回應等功能。
3. 他們的留言會歸屬於他們的 Blackboard 帳戶。若學生之後在 Blackboard 編輯其姓名或照片，下一次啟動時會更新 FastComments 個人檔案。

從 Blackboard 到 FastComments 的角色對應：

- **System Administrator** 和 **Course Builder** 會對應到 FastComments 的 **admin**。
- **Instructor** 和 **Teaching Assistant** 會對應到 FastComments 的 **moderator**。
- **Student**、**Guest** 與 **Observer** 會對應到 FastComments 的 **commenter**。

管理者（moderator）會在留言串中每則留言旁看到內嵌的審核控制（置頂、隱藏、停權、刪除）。

#### 留言串範圍（Thread Scoping）

FastComments 以 **(Blackboard host, course ID, resource link ID)** 作為每個留言串的範圍。相同課程中的兩個 FastComments 項目會產生兩個留言串。相同項目在兩個課程範本間複製（例如透過課程複製）會產生兩個留言串，因為 Blackboard 在複製時會發出新的 resource link ID。若要在課程複製間保留共用留言串，請在進行複製之前使用 Deep Linking 並在 FastComments 中設定明確的 thread URN。

#### Blackboard 專屬常見問題

**Build Content 選單（Original）或 Content Market（Ultra）中找不到 FastComments 方塊。** 管理員已核准該工具，但留下一個機構政策阻擋了相關的放置點。前往 **Administrator Panel** > **Integrations** > **LTI Tool Providers**，編輯 FastComments 條目，並確認已啟用 **Course Content Tool**（Original）以及 **Course Content Tool - allow students** / **Deep Linking content tool**（Ultra）放置。儲存後重新整理課程頁面。

**啟動時出現「Tool not configured for this context」或「Tool is not deployed」錯誤。** 在動態註冊期間註冊的部署範圍（deployment scope）與課程所屬的機構情境不相符。在 Blackboard 的工具提供者條目中，確認 **Deployment ID** 是否與 FastComments 在此租戶的 LTI 1.3 Configuration 頁面中顯示的值相符。如果不同，刪除該放置並從新的註冊 URL 重新執行動態註冊（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>）。

**Iframe 高度看起來固定或內容被裁切。** 有些 Blackboard 租戶帶有嚴格的 Content Security Policy，會阻擋預設的 LTI iframe-resize postMessage。FastComments 同時發送 Canvas 風格的 `lti.frameResize` 訊息與 IMS 規範形式的 `org.imsglobal.lti.frameResize` 訊息以提升相容性，但租戶層級的 CSP 覆寫會阻擋父視窗的監聽器。請你的管理員確認 `*.fastcomments.com` 已在 LTI 工具允許清單上，且沒有自訂 CSP 標頭剝除 postMessage 事件。確認後，調整大小功能即可正常運作，無需進一步設定。

**課程複製會複製留言串。** Blackboard 在課程複製時會為 LTI 放置項目發出新的 resource link ID，因此被複製的課程會從空的留言串開始。這是預期行為。若你需要複製後的課程繼承原始留言串，請在複製前以明確的 thread URN 設定 Deep Linking，或聯絡 FastComments 支援以批次重新對應留言串 ID。

**學生在啟動時看到一般性的 Blackboard 錯誤。** 原因通常是遺失或過期的 `email` claim。確認機構對 FastComments 的政策在 **User Fields to Send** 下已啟用 **Role**、**Name** 與 **Email Address**。儲存後，在新的瀏覽器工作階段中再次啟動。