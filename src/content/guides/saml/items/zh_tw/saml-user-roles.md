FastComments 將 SAML 使用者角色對應到內部權限，為您的組織啟用基於角色的存取控制。

### FastComments 角色系統

FastComments 使用基於角色的權限系統，使用者可擁有一個或多個角色以決定其存取等級與權限。

### 可用的 FastComments 角色

#### 管理角色

**fc-account-owner**
- **Permissions**: 完整的管理存取權限
- **Capabilities**: 所有功能、帳務管理、使用者管理
- **Use Case**: 主要的帳戶管理員與擁有者

**fc-admin-admin**  
- **Permissions**: 大部分功能的管理存取權限
- **Capabilities**: 使用者管理、設定、審核。**可管理其他管理員。**
- **Use Case**: 次要管理員與 IT 人員

**fc-billing-admin**
- **Permissions**: 帳務與訂閱管理
- **Capabilities**: 付款方式、發票、訂閱變更
- **Use Case**: 財務團隊成員與帳務聯絡人

#### 專門角色

**fc-analytics-admin**
- **Permissions**: 存取分析與報告
- **Capabilities**: 檢視網站統計、使用者互動資料
- **Use Case**: 行銷團隊與資料分析師

**fc-api-admin**
- **Permissions**: API 存取與管理
- **Capabilities**: API 憑證、Webhook 設定
- **Use Case**: 開發人員與技術整合者

**fc-moderator**
- **Permissions**: 留言審核權限
- **Capabilities**: 核准/拒絕留言、管理垃圾留言
- **Use Case**: 社群版主與內容管理人員

### 角色映射設定

#### SAML 屬性來源

FastComments 可接受來自不同 SAML 屬性名稱的角色資訊，以確保與不同身分提供者相容：

**標準屬性名稱**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS 屬性**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### 支援的角色格式

**陣列格式** *(首選)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**逗號分隔格式**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**單一角色格式**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### 身分提供者角色設定

#### Microsoft Azure AD

1. **應用程式角色設定**:
   - 在您的 Azure AD 應用中定義 FastComments 角色
   - 將使用者指派到適當的應用角色
   - 設定 claim 以包含已指派的角色

2. **屬性對應**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **群組指派**:
   - 建立與 FastComments 角色名稱相符的群組
   - 將使用者指派到適當的群組
   - 設定屬性聲明

2. **屬性聲明**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **群組對應**:
   - 建立組織單位或群組
   - 使用 FastComments 角色前綴命名群組
   - 設定屬性對應

2. **自訂屬性**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### 使用者的預設行為

#### 沒有角色的使用者

當 SAML 使用者沒有角色或角色無法辨識時：
- 使用者會被建立為一般留言者
- 不會授予任何管理存取權
- 可以發佈並管理自己的留言
- 無法存取管理後台功能

#### 角色繼承

- 使用者可以同時擁有多個角色
- 權限為累加（採用最高權限層級）
- IdP 中的角色變更會在下一次登入時反映

### 管理 SAML 使用者

#### 使用者建立

當使用者第一次透過 SAML 登入時：
1. **使用者帳戶**: 以電子郵件為識別自動建立
2. **角色指派**: 根據 SAML 屬性套用角色
3. **個人資料資訊**: 若提供則填入名字/姓氏
4. **權限啟用**: 角色會立即生效

#### 角色更新

既有的 SAML 使用者會接收角色更新：
1. **登入觸發**: 角色更新在每次 SAML 登入時發生
2. **立即生效**: 新權限會立即套用
3. **角色移除**: 被移除的角色會自動撤銷
4. **稽核追蹤**: 角色變更會記錄在稽核日誌中

### 自訂角色對應

#### 企業自訂化

針對有特定需求的企業客戶：
- 可將自訂角色名稱對應到 FastComments 權限
- 可實作複雜的角色層級
- 可設定部門專屬的存取控制

如需自訂角色對應設定，請聯絡 FastComments 支援。

#### 角色驗證

FastComments 會驗證進入的角色：
- 無法辨識的角色會被忽略（不會被拒絕）
- 格式錯誤的角色屬性會記錄以供故障排除
- 若 SAML 聲明缺少角色資訊，使用者會保留現有角色

### 最佳實務

#### 角色管理

1. **最小權限原則**: 指派最少必要的權限
2. **定期稽核**: 定期檢視使用者角色與存取權  
3. **清晰命名**: 在您的 IdP 中使用具描述性的群組名稱
4. **文件化**: 維護角色指派的文件

#### 安全考量

1. **角色屬性**: 確保 SAML 回應中的角色屬性已妥善保護
2. **屬性驗證**: 驗證只有授權系統能指派角色
3. **存取檢閱**: 定期檢查管理角色的指派
4. **監控**: 監控角色變更與管理操作

### 疑難排解角色問題

#### 常見問題

**角色未套用**:
- 檢查 SAML 屬性名稱是否符合支援格式
- 驗證 IdP 是否發送角色資訊
- 確認角色值與 FastComments 角色名稱完全相符

**存取被拒**:
- 驗證使用者在 IdP 中是否有適當的角色指派
- 檢查角色拼寫與大小寫敏感性
- 確認角色在 SAML 回應中格式正確

**權限遺失**:
- 檢視角色定義與所需權限
- 檢查是否有衝突的角色指派
- 驗證使用者在角色變更後是否已登入過