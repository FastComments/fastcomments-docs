D2L Brightspace 透過 LTI Advantage 管理介面提供動態註冊（Dynamic Registration）。您需要管理員存取權限。

#### 開啟註冊畫面

1. 以管理員身份登入您的 Brightspace 實例。
2. 前往 **Admin Tools** > **Manage Extensibility** > **LTI Advantage**。
3. 點選 **Register Tool**。 (直接 URL 為 `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`。)

#### 貼上 URL

您會看到一個註冊表單。關鍵欄位是 **Tool initiation registration endpoint**（某些 Brightspace 版本會標示為 "Tool Initiation Registration URL"）。

將 FastComments 註冊 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>) 貼到該欄位。其餘欄位留空 - 在註冊握手期間會由 FastComments 自動填入。

點選 **Register**。

#### 核准該工具

Brightspace 會開一個彈出視窗與 FastComments 進行通訊、交換金鑰，並顯示確認畫面。註冊完成後該彈出視窗會自動關閉。

新的工具會出現在您的 LTI Advantage 工具清單中。預設 Brightspace 會將新工具標記為 **disabled** - 將切換按鈕切換為 **enabled**，讓課程可以使用它。

#### 新增部署

在 Brightspace 中，LTI 工具在能夠於課程中使用前需要先有一個 **deployment**：

1. 開啟剛註冊完成的 FastComments 工具。
2. 點選 **View Deployments** > **New Deployment**。
3. 為該部署命名（例如 "FastComments - All Courses"）、選擇該部署應可用的組織單位，然後儲存。

透過此部署首次啟動後，FastComments 會將 `deployment_id` 鎖定到其設定紀錄 - 在相同 client 底下，來自不同部署的後續啟動將會被拒絕，除非您重新註冊。