FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

但是，可能希望在该流程之外管理用户，以提高应用程序的一致性。

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

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
    isAccountOwner?: boolean // 管理权限 - 具有此标志的 SSO 用户将作为 SSO 管理员计费（与常规 SSO 用户分开）
    isAdminAdmin?: boolean // 管理权限 - 具有此标志的 SSO 用户将作为 SSO 管理员计费（与常规 SSO 用户分开）
    isCommentModeratorAdmin?: boolean // 版主权限 - 具有此标志的 SSO 用户将作为 SSO 版主计费（与常规 SSO 用户分开）
    /** 如果为 null，则不会对该用户应用访问控制。如果为空列表，则该用户将无法看到任何页面或 @ 提及其他用户。 **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 不要让其他用户在其个人资料上看到该用户的活动（包括评论）。默认值为 true，以默认提供安全的个人资料。 **/
    isProfileActivityPrivate?: boolean
    /** 不要允许其他用户在该用户的个人资料上留下评论，或查看现有的个人资料评论。默认值为 false。 **/
    isProfileCommentsPrivate?: boolean
    /** 不要允许其他用户向该用户发送直接消息。默认值为 false。 **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** 用户徽章的可选配置。 **/
    badgeConfig?: {
        /** 要分配给用户的徽章 ID 数组。限制为 30 个徽章。顺序将被保留。 **/
        badgeIds: string[]
        /** 如果为 true，则用提供的徽章替换所有现有显示的徽章。如果为 false，则将提供的徽章添加到现有徽章中。 **/
        override?: boolean
        /** 如果为 true，则从租户配置更新徽章显示属性。 **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO users are billed differently based on their permission flags:

- **Regular SSO Users**: Users without admin or moderator permissions are billed as regular SSO users
- **SSO Admins**: Users with `isAccountOwner` or `isAdminAdmin` flags are billed separately as SSO Admins (same rate as regular tenant admins)
- **SSO Moderators**: Users with `isCommentModeratorAdmin` flag are billed separately as SSO Moderators (same rate as regular moderators)

**Important**: To prevent double billing, the system automatically deduplicates SSO users against regular tenant users and moderators by email address. If an SSO user has the same email as a regular tenant user or moderator, they will not be billed twice.

### SSO 用户的计费

根据权限标志，SSO 用户的计费方式不同：

- **常规 SSO 用户**：没有管理员或版主权限的用户按常规 SSO 用户计费
- **SSO 管理员**：具有 `isAccountOwner` 或 `isAdminAdmin` 标志的用户将作为 SSO 管理员单独计费（与常规租户管理员费率相同）
- **SSO 版主**：具有 `isCommentModeratorAdmin` 标志的用户将作为 SSO 版主单独计费（与常规版主费率相同）

**重要**：为防止重复计费，系统会根据电子邮件地址自动对 SSO 用户与常规租户用户和版主进行去重。如果 SSO 用户与常规租户用户或版主的电子邮件相同，则不会被重复计费。

### Access Control

Users can be broken into groups. This is what the `groupIds` field is for, and is optional.

### 访问控制

用户可以被划分为组。这就是 `groupIds` 字段的用途，且为可选。

### @Mentions

By default `@mentions` will use `username` to search for other sso users when the `@` character is typed. If `displayName` is used, then results matching
`username` will be ignored when there is a match for `displayName`, and the `@mention` search results will use `displayName`.

### @提及

默认情况下，当输入 `@` 字符时，`@mentions` 将使用 `username` 来搜索其他 SSO 用户。如果使用 `displayName`，当有与 `displayName` 匹配时，
与 `username` 匹配的结果将被忽略，`@mention` 搜索结果将使用 `displayName`。

### Subscriptions

With FastComments, users can subscribe to a page by clicking the bell icon in the comment widget and clicking Subscribe.

With a regular user, we send them notification emails based on their notification settings.

With SSO Users, we split this up for backwards compatibility. Users will only get sent these additional subscription notification
emails if you set `optedInSubscriptionNotifications` to `true`.

### 订阅

在 FastComments 中，用户可以通过在评论小部件中点击铃铛图标并选择订阅来订阅页面。

对于常规用户，我们会根据他们的通知设置向其发送通知电子邮件。

对于 SSO 用户，我们为向后兼容而进行了拆分。只有当您将 `optedInSubscriptionNotifications` 设置为 `true` 时，用户才会收到这些额外的订阅通知
电子邮件。

### Badges

You can assign badges to SSO users using the `badgeConfig` property. Badges are visual indicators that appear next to a user's name in comments.

- `badgeIds` - An array of badge IDs to assign to the user. These must be valid badge IDs created in your FastComments account. Limited to 30 badges.
- `override` - If true, all existing badges displayed on comments will be replaced with the provided ones. If false or omitted, the provided badges will be added to any existing badges.
- `update` - If true, badge display properties will be updated from the tenant configuration whenever the user logs in.

### 徽章

您可以使用 `badgeConfig` 属性为 SSO 用户分配徽章。徽章是在评论中显示在用户名称旁的视觉指示器。

- `badgeIds` - 要分配给用户的徽章 ID 数组。这些必须是您 FastComments 帐户中创建的有效徽章 ID。限制为 30 个徽章。
- `override` - 如果为 true，则评论中显示的所有现有徽章将被提供的徽章替换。如果为 false 或省略，则将提供的徽章添加到任何现有徽章中。
- `update` - 如果为 true，则在用户登录时将根据租户配置更新徽章显示属性。

---