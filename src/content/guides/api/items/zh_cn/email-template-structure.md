`EmailTemplate` 对象表示租户的自定义电子邮件模板的配置。

系统将通过以下方式选择要使用的电子邮件模板：

- 其类型标识符，我们称之为 `emailTemplateId`。这些是常量。
- `domain`。我们会首先尝试查找与相关对象（例如 `Comment`）关联的域的模板，如果未找到匹配项，则会尝试查找 domain 为 null 或 `*` 的模板。

`EmailTemplate` 对象的结构如下：

[inline-code-attrs-start title = '电子邮件模板结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** 只读 **/
    createdAt: string
    /** 只读 **/
    updatedAt: string
    /** 只读 **/
    updatedByUserId: string
    /** 模板应关联的域。 **/
    domain?: string | '*' | null
    /** 使用 EJS 语法的电子邮件模板内容。 **/
    ejs: string
    /** 针对每个受支持语言环境的覆盖翻译键到值的映射。 **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** 表示模板渲染上下文的对象。 **/
    testData: object
}
[inline-code-end]

### 注意

- 您可以从 `/definitions` 端点获取有效的 `emailTemplateId` 值。
- `/definitions` 端点还包含默认翻译和测试数据。
- 如果结构或测试数据无效，模板将保存失败。

---