#### Sakai

Sakai 支援在具有 LTI Advantage 的版本上使用 LTI 1.3 動態註冊。從 **管理工作區**：

1. 以 Sakai 管理員身分登入並開啟 **管理工作區**。
2. 選擇 **外部工具** > **安裝 LTI 1.3 工具**。
3. 貼上 FastComments 註冊 URL 並提交。
4. 在握手完成後核准該工具。

該工具將出現在 **外部工具** 底下，維護者可以將它新增至網站。

#### Schoology

Schoology Enterprise 實例支援 LTI 1.3，但動態註冊的可用性依部署而異。請洽您的 Schoology 帳戶經理。

如果您的 Schoology 實例無法使用動態註冊，您需要手動使用以下端點來設定整合：

- **OIDC 登入 URL**: `https://fastcomments.com/lti/v1p3/login`
- **目標連結 URL**: `https://fastcomments.com/lti/v1p3/launch`
- **公鑰集 URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **重新導向 URL**: `https://fastcomments.com/lti/v1p3/launch`

在 Schoology 提供您 Client ID 和 Deployment ID 之後，請聯絡 FastComments 支援以在您的租戶上註冊該設定。

#### Other LTI 1.3 Platforms

任何遵循 IMS LTI 1.3 Advantage 規範的 LMS 應可使用相同的註冊 URL。尋找標示為「動態註冊」、「工具註冊 URL」、「工具啟動註冊端點」或類似的設定。

如果您的平台僅支援手動 LTI 1.3 設定，請使用上面 Schoology 區段列出的四個端點，並聯絡支援以完成設定。