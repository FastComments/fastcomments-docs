一旦管理員已將 FastComments 註冊為 LTI 1.3 Advantage 工具並核准機構政策，講師就可以透過標準的 Blackboard 放置點將其新增到課程中。實際步驟在 Ultra Course View 與 Original Course View 之間有所不同，因此下面兩者都會說明。

#### Ultra Course View

Ultra Course View 自 2026 年起為 Blackboard Learn SaaS 的預設。

1. 開啟課程並前往 **Course Content** 頁面。
2. 將游標移到大綱中你想放置留言串的位置，然後點擊紫色的 **+**（新增內容）按鈕。
3. 選擇 **Content Market**。Content Market 面板會列出機構核准的所有 LTI 工具與 Building Block 放置點。
4. 找到 **FastComments** 方塊並點擊它。Blackboard 會在你打開 **+** 選單的位置建立一個內容項目。
5. 預設情況下，對於個人預設為未開啟 **Hide from students** 的講師，該項目會以「Visible to students」條目落在大綱中。如果你的預設為 **Hidden**，該項目會以隱藏方式建立，當你準備好時可以在項目列上切換可見性選取器。
6. 若要重新命名項目，點擊大綱中的標題並輸入新標籤。學生在大綱中看到的標題與 FastComments 線程識別符是獨立的，因此隨時重新命名都是安全的。

如果你看不到 **Content Market** 選項，表示機構已將該放置點隱藏。你也可以透過同一 **+** 選單中的 **More tools** 並在 **LTI Tools** 群組下開啟相同的選擇器。

#### Original Course View

Original Course View 在 Learn SaaS 中仍受支援，並且在 Q4 2024 CU 發行線上的自我架設 Learn 9.1 站點仍是主要體驗。

1. 開啟課程並進入一個 **Content Area**（例如課程選單中的預設 **Information** 或 **Content** 區域）。
2. 在頁面右上角的切換開關將 **Edit Mode** 開啟。
3. 在動作列中點擊 **Build Content**。
4. 在 **Learning Tools** 子選單下，點擊 **FastComments**。Learning Tools 子選單是由管理員註冊工具後的 LTI 1.3 工具放置所填充。如果你看不到它，請參閱下面的常見問題章節。
5. 在 **Create FastComments** 表單中設定：
   - **Name**：學生在內容區看到的標籤。
   - **Description**：顯示於嵌入討論串上方的選用文字。
   - **Permit Users to View this Content**：可用性切換（是/否）。
   - **Track Number of Views**：若你想要 Blackboard 的每項目檢視統計，請啟用。FastComments 會獨立執行自己的分析。
   - **Date and Time Restrictions**：選用的 **Display After** / **Display Until** 時段。
6. 送出。該工具會以可點擊項目的形式顯示在內容區。

#### 從項目或文件內嵌入

在兩種課程檢視中，講師可以透過內容編輯器的 LTI Advantage 按鈕，將 FastComments 內嵌於 Item、Document 或任何富文本欄位的內文中。

Ultra Course View：

1. 建立或編輯 **Document**。
2. 在文件內文中你想顯示討論串的位置點擊 **Add content**。
3. 在編輯器工具列中，開啟 **Insert content** 選單並點擊 **Content Market**（LTI Advantage / Deep Linking 的進入點）。
4. 選擇 **FastComments**。FastComments 會回傳 deep-link 有效負載，Blackboard 將在文件內文中的游標位置插入一個內嵌區塊。
5. 儲存文件。學生在往下滾動時會看到內嵌呈現的討論串。

Original Course View：

1. 編輯任何具有富文本內文的項目。
2. 在內容編輯器工具列中，點擊 **Add Content** 的加號圖示並選擇 **Content Market**（在較舊的 Q4 2024 CU 中標示為 **Add Content from External Tool**）。
3. 選擇 **FastComments**。編輯器會插入一個引用 deep-linked 資源的佔位區塊。
4. 提交該項目。

每個 deep-link 嵌入都會產生其自己的 FastComments 討論串，因此一個包含兩個內嵌 FastComments 區塊的項目會有兩個獨立的留言串。

#### 可見性、發布條件與群組限制

FastComments 內容項目的行為就像任何其他 Blackboard 內容項目一樣，會受到疊加在其上的存取控制規則約束。

- Ultra：在列上點擊可見性選取器（**Visible to students**、**Hidden from students**、**Conditional availability**）。Conditional availability 支援日期/時間視窗、針對成績簿項目的表現規則，以及針對課程群組的成員規則。
- Original：打開該項目的內容選單並選擇 **Adaptive Release** 或 **Adaptive Release: Advanced**，以按日期、成員、成績或審閱狀態限制該工具。使用 **Set Group Availability** 在項目上限制特定課程群組。

FastComments 會遵從 Blackboard 的任何決定。如果 Blackboard 隱藏該項目對某位學生，該學生的 LTI 啟動就不會發生，他們也不會出現在版主檢視中。

#### 成績簿行為

FastComments 不會透過 LTI Advantage Assignment and Grade Services 回傳成績。FastComments 內容項目不會自動建立任何成績欄位。

如果你的 Blackboard 租戶被設定為對每個新內容項目自動建立成績簿欄位（不論是否有評分相關的 metadata），則仍會出現一個空欄位。若要隱藏它：

- Ultra：打開 **Gradebook**，點擊欄位標頭，選擇 **Edit**，並關閉 **Show to students** 以及 **Include in calculations**。或者如果你們機構允許對未評分項目刪除欄位，也可以使用 **Delete**。
- Original：打開 **Grade Center**，點擊該欄位的選單符號，選擇 **Hide from Users (on/off)**，並可選擇在 **Column Organization** 下設定 **Hide from Instructor View**。

#### 學生會看到什麼

當學生開啟 FastComments 項目或捲動到內嵌區塊時：

1. Blackboard 向 FastComments 啟動 LTI 1.3 訊息。學生會透過他們的 Blackboard 身份（姓名、電子郵件、大頭貼、角色）以 SSO 登入，而不會看到登入表單。
2. 討論串在 iframe 中呈現。根據 FastComments 中設定的留言 Widget 選項，會提供分支回覆、回覆、提及與回應等功能。
3. 他們的留言會以其 Blackboard 帳戶歸屬。如果學生之後在 Blackboard 編輯他們的姓名或照片，下一次啟動時會更新 FastComments 個人檔案。

從 Blackboard 到 FastComments 的角色對應：

- **System Administrator** 與 **Course Builder** 對應到 FastComments 的 **admin**。
- **Instructor** 與 **Teaching Assistant** 對應到 FastComments 的 **moderator**。
- **Student**、**Guest** 與 **Observer** 對應到 FastComments 的 **commenter**。

版主會在討論串的每則留言內看到版務控制（置頂、隱藏、封鎖、刪除）。

#### 討論串範圍定義

FastComments 以 **(Blackboard host, course ID, resource link ID)** 定義每個討論串的範圍。相同課程中的兩個 FastComments 項目會產生兩個討論串。相同項目複製到兩個課程殼（例如透過課程複製）會產生兩個討論串，因為 Blackboard 在複製過程中會發出新的 resource link ID。若要在課程複製間保留共用討論串，請在啟動複製之前使用 Deep Linking 並在 FastComments 中設定明確的討論串 URN。

#### Blackboard 專屬常見問題

**Build Content 選單（Original）或 Content Market（Ultra）中找不到 FastComments 方塊。** 管理員已核准該工具，但在機構政策中留下了阻擋相關放置點的設定。前往 **Administrator Panel** > **Integrations** > **LTI Tool Providers**，編輯 FastComments 條目，確認已啟用 **Course Content Tool**（Original）以及 **Course Content Tool - allow students** / **Deep Linking content tool**（Ultra）的放置。儲存並重新整理課程頁面。

**啟動時出現「Tool not configured for this context」或「Tool is not deployed」錯誤。** 在動態註冊期間註冊的部署範圍與該課程所屬的機構情境不相符。於 Blackboard 的工具提供者條目中，確認 **Deployment ID** 與 FastComments 在此租戶的 LTI 1.3 Configuration 頁面所顯示的值相符。若不同，刪除該放置點並從新的註冊 URL 重新執行動態註冊。

**Iframe 高度看起來固定或內容被截斷。** 有些 Blackboard 租戶會隨附嚴格的 Content Security Policy，會阻擋預設的 LTI iframe-resize postMessage。FastComments 會同時發送 Canvas 風格的 `lti.frameResize` 訊息與 IMS 規範形式的 `org.imsglobal.lti.frameResize` 訊息以最大化相容性，但若租戶層級的 CSP 覆寫阻擋了父視窗監聽器，則會失效。請管理員確認 `*.fastcomments.com` 已列在 LTI 工具允許清單中，且沒有自訂 CSP 標頭剝除 postMessage 事件。如此一來，調整大小即可在無需其他設定的情況下運作。

**課程複製會產生重複的討論串。** Blackboard 課程複製會為 LTI 放置發出新的 resource link ID，因此複製的課程會從空白討論串開始。這是預期行為。若你需要複製的課程繼承原始討論串，請在複製前設定 Deep Linking 並指定明確的討論串 URN，或聯絡 FastComments 支援以大量重新對應討論串 ID。

**學生在啟動時看到 Blackboard 的通用錯誤。** 原因通常是缺少或過時的 `email` 宣告（claim）。確認機構對 FastComments 的政策在 **User Fields to Send** 下已啟用 **Role**、**Name** 與 **Email Address**。儲存後，再用新的瀏覽器工作階段重新啟動即可。