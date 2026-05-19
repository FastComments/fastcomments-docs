一旦 FastComments 在平台上註冊完成，教師即可使用平台的標準外部工具流程將其新增到課程內容。本頁涵蓋 Sakai 23.x 與 Schoology Enterprise 的說明。

#### 鎖定公開存取（建議）

預設情況下，FastComments 的留言資料在任一平台上皆可公開讀取。任何能猜到討論串 URL 或 API 端點的人都可以查看其留言，即使不在 Sakai 或 Schoology 內也能看到。對於課程討論，你幾乎肯定會希望將檢視權限限制為僅限已註冊的學生。

打開你的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget 自訂頁面</a>，建立一個規則並啟用 **Require SSO To View Comments**，然後將安全等級設為 **Secure SSO**，使討論串只能透過簽名的 LTI 啟動來載入。

請參閱 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) 取得完整的操作說明，包括如何將規則限定到單一網域或頁面。

#### Sakai

**1. 將 FastComments 新增到站台**

站台維護者可針對每個站台啟用此工具：

1. 開啟該站台並在左側導覽中點選 **Site Info**。
2. 點選 **Manage Tools**。
3. 捲動到 **External Tools** 清單並切換 **FastComments** 為啟用。
4. 點選 **Continue**，檢查工具清單，然後點選 **Finish**。

FastComments 現在會出現在站台的左側導覽項目中。

**2. 重新排序左側導覽項目**

前往 **Site Info** > **Tool Order**。拖動 **FastComments** 到想要的位置，然後點選 **Save**。你也可以在此畫面重新命名導覽標籤並將其對學生隱藏。

**3. 在 Lessons 頁面內嵌入（inline）**

若要將 FastComments 直接放在 Lessons 頁面內，而非獨立的左側工具：

1. 在站台中打開 **Lessons** 工具。
2. 點選 **Add Content** > **Add External Tool**。
3. 從清單中選擇 **FastComments**。
4. 如果 FastComments 在註冊期間宣告了 Deep Linking，Sakai 會開啟工具內容選擇器以便你可以選擇或標記討論串。若未宣告 Deep Linking，Sakai 則會插入預設的啟動連結。
5. 儲存 Lessons 項目。

每個內嵌的實例都會擁有自己的討論串，其範圍綁定在該資源連結上。

**4. 調整學生存取權限**

Sakai 透過 Realms 管理外部工具的啟動。要確認學生可以啟動 FastComments：

1. 以 Sakai 管理員身份登入並打開 **Administration Workspace** > **Realms**。
2. 開啟相關的 realm（例如 `!site.template.course` 或特定的站台 realm）。
3. 確認 `access` 角色已啟用 `lti.launch`，並且在 **external.tools** 群組中的角色權限已被授予。
4. 儲存該 realm。

若要在站台層級進行覆寫，維護者可以從 **Site Info** > **Tool Order** 調整每個角色的工具可見性，透過隱藏或顯示 FastComments 來達成。

**5. 學生會看到的內容**

學生點選 FastComments 左側導覽項目（或捲動到內嵌的 Lessons 區塊）後會直接進入有階層的留言檢視。SSO 是自動的：Sakai 在 LTI 啟動時傳送使用者身分，FastComments 會使用其 Sakai 帳戶為其登入。

角色對應：

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai 注意事項**

- **Manage Tools 中看不到工具。** 若 FastComments 未出現在 External Tools 清單中，Sakai 管理員需要開啟工具登錄（**Administration Workspace** > **External Tools** > **FastComments**）並將 **Stealthed** 設為 `false`。被 stealth 的工具會從每站台的 Manage Tools 選擇器中隱藏。
- **在共用會話瀏覽器中啟動會失敗。** Sakai 的入口網站 CSRF 令牌綁定於瀏覽器會話。如果學生在不同分頁中登入兩個 Sakai 站台或有過期的會話，啟動會返回 403。解決方法：關閉其他 Sakai 分頁，登出，重新登入，然後重新啟動。若在整個叢集普遍發生，管理員也可以提高 `sakai.csrf.token.cache.ttl`。
- **嵌入到框架內。** 請確認 `lti.frameheight` 在 `sakai.properties` 中足夠大（600 或更高），以免討論串在 Lessons 頁面內被截斷。

#### Schoology

Schoology Enterprise 有兩種安裝情境。新增工具到課程前請先確認適用哪一種。

**1. 兩種安裝情境**

- **(a) 企業層級安裝。** Schoology 系統管理員已在組織層級安裝 FastComments，並指派到所有課程或特定的課程範本。此情況下教師可略過安裝步驟，直接前往「Add Materials」。
- **(b) 教師自行安裝。** 教師可從 **Course Options** > **External Tools** > **Install LTI Apps** 將工具安裝到單一課程中。自行安裝前提是系統管理員已先在組織層級核准 FastComments 應用。

**2. 將 FastComments 新增為課程資源（Materials）**

在課程內：

1. 開啟課程並前往 **Materials**。
2. 點選 **Add Materials** > **Add File/Link/External Tool**。
3. 選擇 **External Tool**。
4. 從已註冊的工具清單中選擇 **FastComments**。
5. 設定 **Name**（學生在資源清單中看到的名稱）以及選填的 **Description**。
6. 保持 **Enable Grading**（成績回傳）為 **OFF**。FastComments 不會將成績回報給 Schoology，因此啟用成績回傳會建立一個空的成績欄位。
7. 點選 **Submit**。

該資源現在會出現在課程的資源清單中，點擊時會開啟 FastComments 討論串。

**3. 透過富文字編輯器內嵌（inline）**

若系統管理員在註冊期間為 FastComments 啟用了 Deep Linking 放置位置，教師可以在任何富文字欄位（作業說明、頁面內容、討論提示）內嵌留言討論串：

1. 在目標頁面打開富文字編輯器。
2. 在工具列上點選 **External Tool**（拼圖圖示）按鈕。
3. 選擇 **FastComments**。
4. 在深度連結對話框中設定嵌入，然後點選 **Insert**。
5. 儲存頁面。

若富文字編輯器中未顯示 External Tool 按鈕，表示此租戶對該工具關閉了 Deep Linking。請參閱下方的注意事項。

**4. 可見性與分組（section）指派**

Schoology 透過 Course Options 以分組為單位控制工具可用性：

1. 在課程內，點選 **Course Options** > **External Tools**。
2. 對於每個已安裝的 LTI 應用，你可以控制它是對課程中所有分組可用，或僅對特定分組可用。
3. 若要將 FastComments 限定給某些分組，取消勾選不應看到該工具的分組。
4. 分組層級的存取也會決定哪些分組會在 **Add Materials** > **External Tool** 中看到 FastComments 的項目。

**5. 學生會看到的內容**

學生點選 FastComments 資源（或捲動到內嵌內容）後會進入有階層的討論檢視。SSO 會自動透過 Schoology 的 LTI 啟動並使用他們的 Schoology 帳戶登入。

角色對應：

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology 注意事項**

- **僅限企業版。** 個人與免費的 Schoology 帳戶無法安裝 LTI 1.3 工具。如果你的租戶使用免費方案，Course Options 中將不會有 **External Tools** 選項。請升級到 Schoology Enterprise 以使用 FastComments。
- **租戶預設關閉 Deep Linking。** 部分 Schoology 租戶在組織層級限制 Deep Linking 放置。若發生此情況，教師只會看到 **Add Materials** > **External Tool** 的流程，而不會在富文字編輯器中看到 External Tool 按鈕。要啟用內嵌嵌入，系統管理員需前往 **System Settings** > **Integration** > **LTI 1.3** > **FastComments** 並啟用 **Content Item / Deep Linking** 放置，然後儲存。
- **分組指派的覆寫。** 如果 FastComments 在企業層級已指派但教師在 **Add Materials** 看不到該工具，可能是該課程的分組在組織層級指派中被排除。請系統管理員將該分組加入 FastComments 應用指派。
- **資源名稱與討論串識別。** 在 Schoology 中重新命名資源不會移動討論串。討論串是以 LTI resource link ID 作為鍵值，因此重新命名會保留相同的討論串；刪除並重新建立資源會建立新的空討論串。