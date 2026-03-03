FastComments 提供了一个易于使用的 SSO 解决方案。通过基于 HMAC 的集成更新用户信息是
像让用户使用更新后的负载加载页面一样简单。

但是，可能希望在该流程之外管理用户，以提高应用的一致性。

SSO 用户 API 提供了一种对我们称之为 SSOUsers 的对象进行 CRUD 的方式。这些对象不同于常规 Users 并且
为类型安全而分开保存。

SSOUser 对象的结构如下：

[inline-code-attrs-start title = 'SSOUser 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // 管理权限 - 带有此标志的 SSO 用户按 SSO 管理员计费（与常规 SSO 用户分开）
    isAdminAdmin?: boolean // 管理权限 - 带有此标志的 SSO 用户按 SSO 管理员计费（与常规 SSO 用户分开）
    isCommentModeratorAdmin?: boolean // 管理员权限 - 带有此标志的 SSO 用户按 SSO 管理员（版主）计费（与常规 SSO 用户分开）
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. These are global badges visible on all pages. **/
        badgeIds: string[]
        /** Array of badge IDs scoped to the current page (urlId). These badges are only displayed on the page where they were assigned. **/
        pageBadgeIds?: string[]
        /** If true, replaces all existing displayed badges with the provided ones. Global and page-scoped badges are overridden independently. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### SSO 用户计费

SSO 用户的计费基于其权限标志有所不同：

- **常规 SSO 用户**：没有管理员或版主权限的用户按常规 SSO 用户计费
- **SSO 管理员**：具有 `isAccountOwner` 或 `isAdminAdmin` 标志的用户作为 SSO 管理员单独计费（费率与常规租户管理员相同）
- **SSO 版主**：具有 `isCommentModeratorAdmin` 标志的用户作为 SSO 版主单独计费（费率与常规版主相同）

**重要**：为防止重复计费，系统会根据电子邮件地址自动对 SSO 用户与常规租户用户和版主进行去重。如果 SSO 用户与常规租户用户或版主具有相同的电子邮件，则不会重复计费。

### 访问控制

用户可以被划分到组中。这就是 `groupIds` 字段的用途，且该字段为可选。

### @提及

默认情况下，当输入 `@` 字符时，`@mentions` 将使用 `username` 来搜索其他 sso 用户。如果使用 `displayName`，则当存在与 `displayName` 匹配的结果时，会忽略与 `username` 的匹配，`@mention` 的搜索结果将使用 `displayName`。

### 订阅

使用 FastComments，用户可以通过点击评论小部件中的铃铛图标并点击订阅来订阅页面。

对于常规用户，我们会根据他们的通知设置向他们发送通知电子邮件。

对于 SSO 用户，我们为了向后兼容将其分开处理。只有当您将 `optedInSubscriptionNotifications` 设置为 `true` 时，用户才会收到这些额外的订阅通知电子邮件。

### 徽章

您可以使用 `badgeConfig` 属性为 SSO 用户分配徽章。徽章是在评论中显示在用户名称旁边的视觉指示。

- `badgeIds` - 要分配给用户的徽章 ID 数组。这些是全局徽章，在所有页面上可见。必须是您在 FastComments 帐户中创建的有效徽章 ID。限制为 30 个徽章。
- `pageBadgeIds` - 可选的徽章 ID 数组，作用域为当前页面（`urlId`）。这些徽章仅在分配它们的页面上显示。不同页面可以为同一用户设置不同的页面作用域徽章。
- `override` - 如果为 true，则会用提供的徽章替换所有现有显示的徽章。全局徽章和页面作用域徽章独立覆盖 —— 覆盖全局徽章不会影响页面作用域徽章，反之亦然。如果为 false 或省略，则会将提供的徽章添加到现有徽章中。
- `update` - 如果为 true，则在用户登录时会从租户配置更新徽章显示属性。

---