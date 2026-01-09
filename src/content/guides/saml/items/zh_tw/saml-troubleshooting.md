本指南涵蓋常見的 SAML 驗證問題及其解決方法。

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- "Certificate validation failed" 錯誤
- 使用者無法完成 SAML 驗證
- SAML 回應被拒絕

**Common Causes**:
- 憑證格式不正確
- 憑證已過期
- 提供了錯誤的憑證
- 憑證中有額外字元或空白

**Solutions**:
1. **Verify Certificate Format**:
   - 確認憑證包含 `-----BEGIN CERTIFICATE-----` 和 `-----END CERTIFICATE-----` 標記
   - 移除任何多餘的空白或換行
   - 直接從 IdP 的 metadata 或設定中複製憑證

2. **Check Certificate Validity**:
   - 驗證憑證尚未過期
   - 確認憑證屬於正確的 IdP
   - 使用線上憑證驗證工具檢查格式

3. **Re-download Certificate**:
   - 從 IdP 重新下載新的憑證
   - 若可用，使用 IdP metadata URL
   - 確認憑證與目前的 IdP 設定相符

#### Signature Verification Failed

**Symptoms**:
- SAML assertion 簽章驗證錯誤
- 在 IdP 登入後驗證失敗
- 出現 "Invalid signature" 錯誤訊息

**Solutions**:
1. **Algorithm Mismatch**:
   - 檢查 FastComments 中的簽章演算法是否與 IdP 相符
   - 嘗試不同的簽章演算法（SHA-256、SHA-1、SHA-512）
   - 驗證摘要演算法是否與 IdP 設定匹配

2. **Certificate Issues**:
   - 確保已配置正確的簽章憑證
   - 驗證憑證是否對應 IdP 使用的私鑰
   - 檢查 IdP 是否有憑證輪換

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- IdP 報告 "Unknown Service Provider"
- SAML 回應發送到錯誤的端點
- 驗證無法完成

**Solutions**:
1. **Verify SP Information**:
   - 從 FastComments 設定複製完整且精確的 Entity ID
   - 確保 ACS URL 符合格式：`https://fastcomments.com/saml/callback/{tenant-id}`
   - 檢查 tenant ID 是否有拼寫錯誤

2. **IdP Configuration**:
   - 在 IdP 中更新為正確的 SP Entity ID
   - 設定正確的 ACS/Reply URL
   - 驗證 IdP 的 binding 設定（建議使用 HTTP-POST）

#### Missing or Incorrect Attributes

**Symptoms**:
- 使用者建立時沒有正確的角色
- 遺失使用者設定檔資訊
- 出現 "Email required" 錯誤

**Solutions**:
1. **Email Attribute**:
   - 確保 IdP 傳送 email 屬性
   - 檢查屬性名稱映射（email、emailAddress 等）
   - 驗證 email 值為有效的電子郵件地址

2. **Role Attributes**:
   - 確認 IdP 傳送角色/群組資訊
   - 檢查角色屬性名稱是否與 FastComments 的預期相符
   - 驗證角色值完全符合 FastComments 的角色名稱

3. **Attribute Format**:
   - 測試陣列與逗號分隔的角色格式
   - 確保屬性值沒有多餘空白
   - 檢查角色名稱是否有大小寫敏感問題

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- 瀏覽器在 FastComments 與 IdP 之間無限重導
- 驗證永遠無法完成
- 在瀏覽器開發工具中看到多次重導

**Solutions**:
1. **Check SP Configuration**:
   - 驗證 Entity ID 與 IdP 設定完全相符
   - 確保 ACS URL 在 IdP 中正確設定
   - 檢查 URL 是否有尾端斜線差異

2. **Session Issues**:
   - 清除瀏覽器 Cookies 並重試
   - 在無痕/私人視窗中測試
   - 檢查會話逾時設定

#### Access Denied After Authentication

**Symptoms**:
- SAML 驗證成功
- 使用者被導回 FastComments
- 顯示 "Access denied" 或權限錯誤

**Solutions**:
1. **Role Assignment**:
   - 驗證使用者在 IdP 中是否有適當的角色
   - 檢查角色屬性是否在 SAML 回應中傳送
   - 確認角色名稱與 FastComments 要求完全相符

2. **Package Limitations**:
   - 驗證帳戶是否為 Flex 或 Pro 套餐
   - 檢查 SAML 功能是否已為該套件啟用
   - 若套件包含 SAML 但功能無法使用，請聯絡支援

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- App role 指派未反映在 token 中
- claims 未正確傳送
- 需要使用者指派

**Solutions**:
- 檢查使用者是否已指派到 FastComments 應用程式
- 驗證 app role 是否正確設定
- 確保 claims 映射包含所需屬性

#### Okta

**Common Issues**:
- 群組過濾器未正常運作
- 屬性聲明設定錯誤
- 應用程式指派問題

**Solutions**:
- 檢視屬性聲明設定
- 檢查群組指派與過濾規則
- 驗證應用程式是否指派給適當的使用者/群組

#### Google Workspace

**Common Issues**:
- 自訂屬性未正確映射
- 群組成員資格未傳送
- SAML 應用程式設定錯誤

**Solutions**:
- 為角色屬性設定自訂 schema
- 檢查群組成員資格傳播
- 驗證 SAML 應用程式的屬性映射

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- 驗證流程逾時
- 出現 "Request timeout" 或類似錯誤
- 驗證流程緩慢

**Solutions**:
1. **Network Connectivity**:
   - 檢查防火牆規則是否允許 FastComments 通訊
   - 驗證 fastcomments.com 的 DNS 解析
   - 從 IdP 測試到 FastComments 的網路連線

2. **Performance Issues**:
   - 檢查 IdP 的回應時間
   - 驗證憑證鏈驗證是否花費過久
   - 考量 IdP 與使用者之間的網路延遲

#### SSL/TLS Issues

**Symptoms**:
- 在驗證期間出現憑證警告
- SSL 握手失敗
- 出現 "Secure connection failed" 錯誤

**Solutions**:
- 確保所有 SAML 端點使用 HTTPS
- 檢查所有相關網域的憑證有效性
- 驗證 TLS 版本相容性

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - 在 SAML 流程中監控 Network 標籤
   - 檢查 Console 是否有 JavaScript 錯誤
   - 檢視 SAML POST 請求（若可見）

2. **IdP Logging**:
   - 在 IdP 中啟用 SAML 偵錯
   - 檢閱 IdP 日誌中的 SAML 請求/回應細節
   - 檢查屬性映射問題

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - 未啟用 SAML 或設定錯誤
- "Invalid certificate" - 憑證驗證失敗
- "Missing email attribute" - SAML 回應中未提供必要的 email

**IdP Logs**:
- "Unknown service provider" - Entity ID 不符
- "Invalid ACS URL" - Assertion Consumer Service URL 錯誤
- "User not assigned" - 使用者未被指派訪問 SAML 應用程式

### Getting Help

#### Information to Gather

聯絡支援時請提供：
- 精確的錯誤訊息與時間戳記
- SAML 設定細節（不包括敏感資料）
- IdP 類型與版本
- 重現問題的步驟
- 瀏覽器與網路資訊

#### FastComments Support

針對 SAML 相關問題：
1. 使用 [support portal](https://fastcomments.com/auth/my-account/help)
2. 包含 tenant ID 和受影響的使用者電子郵件
3. 提供錯誤訊息與設定細節
4. 指定 IdP 類型與設定方式

#### IdP Support

針對 IdP 特定問題：
- 參閱 IdP 的文件以取得 SAML 疑難排解資訊
- 使用 IdP 的支援管道處理設定問題
- 利用 IdP 的社群論壇尋找常見問題解決方案

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - 在非生產環境測試設定變更
   - 使用多個測試使用者驗證
   - 記錄可用的運作設定

2. **Monitor Regularly**:
   - 建立 SAML 驗證失敗的監控
   - 檢閱憑證到期日
   - 監控 IdP 配置變更

3. **Documentation**:
   - 維護 SAML 設定文件
   - 記錄任何自訂設定或變通方法
   - 保留 IdP 管理員的聯絡資訊

#### Proactive Maintenance

1. **Certificate Management**:
   - 監控憑證到期日
   - 規劃憑證輪換程序
   - 在憑證到期前測試憑證更新

2. **Configuration Reviews**:
   - 定期檢閱 SAML 設定
   - 驗證 IdP 設定是否仍為最新
   - 在變更時更新文件