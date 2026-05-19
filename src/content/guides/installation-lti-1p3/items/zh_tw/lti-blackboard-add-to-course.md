一旦管理員已將 FastComments 註冊為 LTI 1.3 Advantage 工具並核准機構政策，講師便可透過 Blackboard 的標準放置點將其加入課程。Ultra Course View 和 Original Course View 的具體步驟不同，所以下文兩種情形皆有說明。

#### Ultra Course View

Ultra Course View 自 2026 年起為 Blackboard Learn SaaS 的預設。

1. 開啟課程並前往 **Course Content** 頁面。
2. 將滑鼠移到或點選大綱中你希望留言串出現的位置，然後點擊紫色的 **+**（新增內容）按鈕。
3. 選擇 **Content Market**。Content Market 面板會列出機構核准的所有 LTI 工具與 Building Block 放置點。
4. 找到 **FastComments** 磚塊並點擊。Blackboard 會在你打開 **+** 選單的位置建立一個內容項目。
5. 該項目預設會以「對學生可見」的條目出現在大綱中，假如講師的個人預設沒有啟用 **Hide from students**。如果你的預設為 **Hidden**，項目會以隱藏狀態建立，等你準備好時可在項目列上切換可見性選取器。
6. 若要重新命名項目，請點擊大綱中的標題並輸入新標籤。學生在大綱中看到的標題與 FastComments 線程識別碼是獨立的，所以隨時重新命名都是安全的。

如果你沒看到 **Content Market** 選項，代表機構已將該放置點隱藏。你也可以在同一個 **+** 選單下的 **More tools** 中，於 **LTI Tools** 群組找到相同的選取器。

#### Original Course View

Original Course View 在 Learn SaaS 中仍受支援，並且在 Q4 2024 CU 發行線上的自我架設 Learn 9.1 站點仍是主要體驗。

1. 開啟課程並進入一個 **Content Area**（例如課程選單中的預設 **Information** 或 **Content** 區域）。
2. 在頁面右上方用切換開啟 **Edit Mode**。
3. 在動作列中點擊 **Build Content**。
4. 在 **Learning Tools** 子選單下點擊 **FastComments**。Learning Tools 子選單會在管理員註冊工具後，從 LTI 1.3 工具放置中填充。如果你沒看到它，請參閱下方的注意事項章節。
5. 在 **Create FastComments** 表單上設定：
   - **Name**：學生在內容區看到的標籤。
   - **Description**：顯示在嵌入討論串上方的選用文字。
   - **Permit Users to View this Content**：是/否 的可用性切換。
   - **Track Number of Views**：若想要 Blackboard 的每項目檢視統計，請啟用。本身 FastComments 會獨立執行分析。
   - **Date and Time Restrictions**：選用的 **Display After** / **Display Until** 時間窗。
6. 送出。該工具會以可點擊項目的形式出現在內容區。

#### 嵌入在項目或文件內

在兩種課程檢視中，講師都可以透過內容編輯器的 LTI Advantage 按鈕，將 FastComments 內嵌於項目、文件或任何富文本欄位的主體內。

Ultra Course View：

1. 建立或編輯一個 **Document**。
2. 在文件主體內想要留言串出現的位置點擊 **Add content**。
3. 在編輯器工具列中打開 **Insert content** 選單並點擊 **Content Market**（LTI Advantage / Deep Linking 的進入點）。
4. 選取 **FastComments**。FastComments 回傳 deep-link 載荷，Blackboard 會在游標位置於文件主體中插入一個嵌入區塊。
5. 儲存文件。學生在捲動經過時會看到內嵌呈現的留言串。

Original Course View：

1. 編輯任何具有富文本主體的項目。
2. 在內容編輯器工具列中，點擊 **Add Content** 的加號圖示並選擇 **Content Market**（在較舊的 Q4 2024 CU 中標示為 **Add Content from External Tool**）。
3. 選取 **FastComments**。編輯器會插入一個參考 deep-linked 資源的佔位區塊。
4. 送出該項目。

每次 deep-link 嵌入都會產生自己的 FastComments 線程，因此一個項目若有兩個嵌入的 FastComments 區塊，就會有兩個獨立的留言串。

#### 可見性、發布條件與群組限制

FastComments 的內容項目在其上所套用的存取控制規則方面，行為與 Blackboard 的其他內容項目相同。

- Ultra：在列上點擊可見性選取器（**Visible to students**、**Hidden from students**、**Conditional availability**）。Conditional availability 支援日期/時間窗、針對成績簿項目的表現規則，以及針對課程群組的成員規則。
- Original：開啟項目的內容選單並選擇 **Adaptive Release** 或 **Adaptive Release: Advanced**，以依日期、成員資格、成績或審閱狀態限制工具。使用項目的 **Set Group Availability** 可限制至特定課程群組。

FastComments 會尊重 Blackboard 的任何限制決定。如果 Blackboard 對某學生隱藏該項目，該學生就不會進行 LTI 啟動，且不會出現在管理者檢視中。

#### 成績簿行為

FastComments 不會透過 LTI Advantage Assignment and Grade Services 回報成績。FastComments 內容項目不會自動建立成績欄位。

如果你的 Blackboard 租戶設定為對每個新內容項目自動建立成績欄（不論是否含分數元資料），則仍會出現一個空白欄。要將其隱藏：

- Ultra：打開 **Gradebook**，點擊欄位標題，選擇 **Edit**，然後關閉 **Show to students** 與 **Include in calculations**。或者在你的機構允許對非評分項目刪除欄位時使用 **Delete**。
- Original：打開 **Grade Center**，點擊欄位的下拉箭頭，選擇 **Hide from Users (on/off)**，並可在 **Column Organization** 下選擇 **Hide from Instructor View**。

#### 學生會看到什麼

當學生開啟 FastComments 項目或捲動到內嵌區塊時：

1. Blackboard 會發送 LTI 1.3 訊息給 FastComments。學生會使用其 Blackboard 身份（名稱、電子郵件、頭像、角色）透過 SSO 登入，而不會看到登入表單。
2. 討論串會在 iframe 中呈現。依據在 FastComments 中配置的留言小工具設定，會提供分線、回覆、提及與表情等功能。
3. 他們的留言會歸屬於其 Blackboard 帳戶。如果學生日後在 Blackboard 編輯其名稱或照片，下一次啟動時會更新 FastComments 個人檔案。

從 Blackboard 到 FastComments 的角色對應：

- **System Administrator** 與 **Course Builder** 對應為 FastComments 的 **admin**。
- **Instructor** 與 **Teaching Assistant** 對應為 FastComments 的 **moderator**。
- **Student**、**Guest** 與 **Observer** 對應為 FastComments 的 **commenter**。

管理者（moderator）會在每則留言內看到內嵌的審查控制（置頂、隱藏、封鎖、刪除）。

#### 鎖定公開存取（建議）

預設情況下，FastComments 的留言資料是公開可讀的。任何能猜到線程 URL 或 API 端點的人都能查看其留言，即使是在 Blackboard 之外。對於課堂討論，你幾乎肯定會希望將檢視限制為僅限已註冊的學生。

開啟你的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a>，建立一個規則並啟用 **Require SSO To View Comments**，然後將安全等級設為 **Secure SSO**，使線程只能透過已簽署的 LTI 啟動載入。

完整操作步驟請參見 [保護留言串使用單一登入](/guide-customizations-and-configuration.html#sso-require-to-view-comments)。

#### 線程範圍設定

FastComments 會以 **(Blackboard host, course ID, resource link ID)** 來範圍化每個線程。同一課程中的兩個 FastComments 項目會產生兩個線程。相同項目若被複製到兩個課程外殼（例如透過課程複製），會產生兩個線程，因為 Blackboard 在複製時會發出新的 resource link ID。若要在課程複製間保留共用線程，請在複製前使用 Deep Linking 並在 FastComments 中為該線程設定明確的 thread URN。

#### Blackboard 專屬注意事項

**Build Content 選單（Original）或 Content Market（Ultra）中缺少 FastComments 磚塊。** 管理員核准了工具，但在機構政策中遺漏了相關放置點。前往 **Administrator Panel** > **Integrations** > **LTI Tool Providers**，編輯 FastComments 條目，確認已啟用 **Course Content Tool**（Original）以及 **Course Content Tool - allow students** / **Deep Linking content tool**（Ultra）放置。儲存並重新整理課程頁面。

**啟動時出現「Tool not configured for this context」或「Tool is not deployed」錯誤。** 在動態註冊時登記的部署範圍與課程所屬的機構情境不符。於 Blackboard 的工具提供者條目中，確認 **Deployment ID** 與 FastComments 在此租戶的 LTI 1.3 Configuration 頁面顯示的值相符。如果兩者不同，刪除該放置點並從新的註冊 URL 重新執行動態註冊（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>）。

**Iframe 高度看起來固定或內容被截斷。** 有些 Blackboard 租戶會帶有嚴格的 Content Security Policy，阻擋預設的 LTI iframe-resize postMessage。為了最大相容性，FastComments 同時發送 Canvas 風格的 `lti.frameResize` 訊息與 IMS 規範形式的 `org.imsglobal.lti.frameResize` 訊息，但租戶層級的 CSP 覆蓋會阻擋父視窗的監聽器。請要求你的管理員確認 `*.fastcomments.com` 已在 LTI 工具的允許清單上，並且沒有自訂 CSP 標頭剝除 postMessage 事件。確認後調整大小功能即可在無需其他設定的情況下運作。

**課程複製會造成線程重複。** Blackboard 在課程複製時會為 LTI 放置發出新的 resource link ID，因此複製後的課程會以空白線程開始。這是預期行為。若你需要複製後的課程繼承原始線程，請在複製前設定帶有明確線程 URN 的 Deep Linking，或聯絡 FastComments 支援以大量重映射線程 ID。

**學生在啟動時看到 Blackboard 的一般錯誤。** 原因通常是缺少或過期的 `email` claim。確認機構對 FastComments 的政策在 **User Fields to Send** 下已啟用 **Role**、**Name** 與 **Email Address**。儲存後在新的瀏覽器分頁中重新啟動即可。