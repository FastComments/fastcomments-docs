#### Sakai

Sakai 在具備 LTI Advantage 的版本中支援 LTI 1.3 動態註冊。請從 Administration Workspace：

1. 以 Sakai 管理員身分登入，並開啟 **Administration Workspace**。
2. 選擇 **External Tools** > **Install LTI 1.3 Tool**。
3. 貼上 FastComments 註冊網址（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">由此取得</a>）並提交。
4. 在交握完成後核准該工具。

該工具隨後會出現在 **External Tools** 下方，並可由網站維護者新增至網站。

#### Schoology

Schoology Enterprise 執行個體支援 LTI 1.3，但動態註冊的可用性因部署而異。請向您的 Schoology 客戶經理確認。

如果您的 Schoology 執行個體不支援動態註冊，您需要使用下列端點手動設定整合：

- **OIDC 登入網址**：`https://fastcomments.com/lti/v1p3/login`
- **目標連結網址**：`https://fastcomments.com/lti/v1p3/launch`
- **公開金鑰集網址（JWKS）**：`https://fastcomments.com/lti/v1p3/jwks`
- **重新導向網址**：`https://fastcomments.com/lti/v1p3/launch`

在 Schoology 提供您 Client ID 與 Deployment ID 之後，請聯絡 FastComments 支援團隊，以在您的租戶上註冊該設定。

#### 其他 LTI 1.3 平台

任何遵循 IMS LTI 1.3 Advantage 規範的 LMS 都應可使用相同的註冊網址（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">由此取得</a>）。請尋找標示為「Dynamic Registration」、「Tool Registration URL」、「Tool initiation registration endpoint」或類似名稱的設定。

如果您的平台僅支援手動 LTI 1.3 設定，請使用上方 Schoology 章節所列的四個端點，並聯絡支援團隊以完成設定。