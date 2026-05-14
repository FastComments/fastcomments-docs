#### Sakai

Sakai 在包含 LTI Advantage 的發行版本中支援 LTI 1.3 的動態註冊。從 **管理工作區**：

1. 以 Sakai 管理員身分登入並開啟 **管理工作區**。
2. 選擇 **External Tools** > **Install LTI 1.3 Tool**。
3. 貼上 FastComments 的註冊 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>) 並提交。
4. 在握手完成後核准該工具。

該工具會出現在 **External Tools** 下，網站維護者即可將其新增至網站。

#### Schoology

Schoology Enterprise 實例支援 LTI 1.3，但是否提供動態註冊取決於部署。請向您的 Schoology 客戶經理確認。

如果您的 Schoology 實例不提供動態註冊，您需要使用以下端點手動設定整合：

- **OIDC 登入 URL**: `https://fastcomments.com/lti/v1p3/login`
- **目標連結 URL**: `https://fastcomments.com/lti/v1p3/launch`
- **公開金鑰集 URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **重新導向 URL**: `https://fastcomments.com/lti/v1p3/launch`

在 Schoology 提供您 Client ID 和 Deployment ID 後，請聯絡 FastComments 支援以在您的租戶上註冊此設定。

#### Other LTI 1.3 Platforms

任何遵循 IMS LTI 1.3 Advantage 規範的 LMS 應該都能使用相同的註冊 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>)。請尋找標示為「Dynamic Registration」、「Tool Registration URL」、「Tool initiation registration endpoint」或類似名稱的設定。

如果您的平台僅支援手動 LTI 1.3 設定，請使用上面 Schoology 區段列出的四個端點，並聯絡支援以完成設定。