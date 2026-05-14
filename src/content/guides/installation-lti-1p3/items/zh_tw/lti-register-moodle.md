**使用 Moodle 嗎？** 我們也提供專用的 FastComments Moodle 外掛，比 LTI 1.3 提供更緊密的整合（成績同步掛鉤、更深度的活動報告、原生的 Moodle 設定介面）。請參閱 <a href="/guide-installation-moodle.html" target="_blank">Moodle 外掛安裝指南</a>。如果您想要一次註冊同時涵蓋其他 LMS，或是您的 Moodle 管理員不會安裝第三方外掛，下列的 LTI 1.3 流程會是正確的選擇。

Moodle 4.0+ 支援透過 External Tool 外掛進行 LTI 1.3 動態註冊。

#### 開啟工具管理畫面

1. 以網站管理員身分登入 Moodle。
2. 前往 **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**。

#### 貼上 URL

您會看到一個標示為 **Tool URL** 的卡片。將 FastComments 註冊 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>）貼到文字欄位，然後點選 **Add LTI Advantage**。

Moodle 會開啟一個註冊畫面，顯示此工具的身分以及它要求的權限。檢查後點選 **Activate**（或視 Moodle 版本為 **Register**）。

註冊完成後彈出視窗會關閉；新的 FastComments 工具會出現在 **Tools** 清單中，狀態為 **Active**。

#### 使其可用

預設情況下 Moodle 會將新工具加入「Course tools」清單，但不會在活動選擇器中顯示。要在整個課程中顯示 FastComments：

1. 點選 FastComments 欄位上的齒輪圖示。
2. 在 **Tool configuration usage** 下，選擇 **Show in activity chooser and as a preconfigured tool**。
3. 儲存。

教師現在可以透過 **Add an activity or resource** > **FastComments** 將 FastComments 新增到任何課程中。