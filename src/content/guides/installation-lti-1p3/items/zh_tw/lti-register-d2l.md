D2L Brightspace 透過 LTI Advantage 管理介面提供動態註冊。您需要管理員權限。

#### 開啟註冊畫面

1. 以管理員身分登入您的 Brightspace 實例。
2. 導覽至 **管理工具** > **管理擴充性** > **LTI Advantage**。
3. 點選 **註冊工具**。 (直接 URL 為 `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`。)

#### 貼上網址

您會看到註冊表單。關鍵欄位是 **Tool initiation registration endpoint**（某些 Brightspace 版本會標示為 "Tool Initiation Registration URL"）。

將 FastComments 的註冊 URL 貼到該欄位。其餘欄位請留空 — FastComments 會在註冊握手期間自動填入。

點選 **註冊**。

#### 核准工具

Brightspace 會開啟一個快顯視窗與 FastComments 進行通訊、交換金鑰，並顯示確認畫面。註冊完成後快顯視窗會自動關閉。

新工具會出現在您的 LTI Advantage 工具清單中。預設 Brightspace 會將新工具標記為 **停用** — 將切換開關切換為 **啟用**，以便您的課程可以使用它。

#### 新增部署

在 Brightspace 中，LTI 工具需要一個 **部署** 才能在課程中使用：

1. 開啟剛註冊的 FastComments 工具。
2. 點選 **檢視部署** > **新增部署**。
3. 為該部署命名（例如 "FastComments - All Courses"），選取該部署應該可用的組織單位，然後儲存。

在透過此部署首次啟動後，FastComments 會將 `deployment_id` 鎖定到其組態記錄中 - 來自同一 client 下其他部署的後續啟動會被拒絕，除非您重新註冊。