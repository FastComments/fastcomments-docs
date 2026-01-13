FastComments 将 SAML 用户角色映射到内部权限，为您的组织启用基于角色的访问控制。

### FastComments 角色系统

FastComments 使用基于角色的权限系统，用户可以拥有一个或多个角色，这些角色决定他们的访问级别和功能权限。

### 可用的 FastComments 角色

#### 管理角色

**fc-account-owner**
- **Permissions**: 完整的管理访问权限
- **Capabilities**: 所有功能、计费管理、用户管理
- **Use Case**: 主要的帐户管理员和所有者

**fc-admin-admin**  
- **Permissions**: 对大多数功能的管理访问权限
- **Capabilities**: 用户管理、配置、审核。**Can administer other admins.**
- **Use Case**: 次要管理员和 IT 人员

**fc-billing-admin**
- **Permissions**: 计费和订阅管理
- **Capabilities**: 支付方式、发票、订阅更改
- **Use Case**: 财务团队成员和计费联系人

#### 专用角色

**fc-analytics-admin**
- **Permissions**: 访问分析和报告
- **Capabilities**: 查看网站统计、用户参与数据
- **Use Case**: 市场团队和数据分析师

**fc-api-admin**
- **Permissions**: API 访问和管理
- **Capabilities**: API 凭证、Webhook 配置
- **Use Case**: 开发人员和技术集成人员

**fc-moderator**
- **Permissions**: 评论审核权限
- **Capabilities**: 批准/拒绝评论、管理垃圾评论
- **Use Case**: 社区版主和内容管理员

### 角色映射配置

#### SAML 属性来源

FastComments 接受来自各种 SAML 属性名称的角色信息，以确保与不同身份提供商的兼容性：

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### 角色格式支持

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### 身份提供商角色配置

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - 在您的 Azure AD 应用中定义 FastComments 角色
   - 将用户分配到相应的应用角色
   - 配置声明以包含已分配的角色

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - 创建与 FastComments 角色名称匹配的组
   - 将用户分配到相应的组
   - 配置属性声明

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - 创建组织单位或组
   - 使用 FastComments 角色前缀命名组
   - 配置属性映射

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### 默认用户行为

#### 无角色的用户

当 SAML 用户没有角色或角色无法识别时：
- 用户将被创建为普通评论者
- 不授予任何管理访问权限
- 可以发布并管理自己的评论
- 无法访问管理员控制面板功能

#### 角色继承

- 用户可以同时拥有多个角色
- 权限是累加的（适用最高权限级别）
- IdP 中的角色更改将在下一次登录时反映

### 管理 SAML 用户

#### 用户创建

当用户通过 SAML 首次登录时：
1. **User Account**: 自动使用电子邮件作为标识创建
2. **Role Assignment**: 根据 SAML 属性应用角色
3. **Profile Information**: 如果提供，则填充名/姓
4. **Permission Activation**: 角色会立即生效

#### 角色更新

现有 SAML 用户接收角色更新：
1. **Login Trigger**: 角色更新在每次 SAML 登录期间发生
2. **Immediate Effect**: 新权限立即生效
3. **Role Removal**: 被移除的角色会被自动撤销
4. **Audit Trail**: 角色更改记录在审计日志中

### 自定义角色映射

#### 企业定制

针对具有特定需求的企业客户：
- 可以将自定义角色名称映射到 FastComments 权限
- 可以实现复杂的角色层级
- 可以配置部门特定的访问控制

联系 FastComments 支持以获取自定义角色映射配置。

#### 角色验证

FastComments 验证传入的角色：
- 无法识别的角色将被忽略（不会被拒绝）
- 格式错误的角色属性将被记录以便故障排除
- 如果 SAML 断言缺少角色信息，用户将保留现有角色

### 最佳实践

#### 角色管理

1. **Principle of Least Privilege**: 分配最小必要权限
2. **Regular Auditing**: 定期审查用户角色和访问权限  
3. **Clear Naming**: 在您的 IdP 中使用描述性组名
4. **Documentation**: 维护角色分配的文档

#### 安全注意事项

1. **Role Attributes**: 确保在 SAML 响应中适当保护角色属性
2. **Attribute Validation**: 验证只有授权系统可以分配角色
3. **Access Reviews**: 定期审查管理角色分配
4. **Monitoring**: 监控角色更改和管理操作

### 角色问题排查

#### 常见问题

**Roles Not Applied**:
- 检查 SAML 属性名称是否与支持的格式匹配
- 验证 IdP 是否发送角色信息
- 确认角色值是否与 FastComments 角色名称完全匹配

**Access Denied**:
- 验证用户在 IdP 中是否分配了适当的角色
- 检查角色拼写和大小写敏感性
- 确认角色在 SAML 响应中格式正确

**Missing Permissions**:
- 审查角色定义和所需权限
- 检查是否存在冲突的角色分配
- 验证用户在角色更改后是否已登录过一次

---