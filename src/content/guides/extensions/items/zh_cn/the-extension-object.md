扩展对象由以下定义组成：

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = '扩展对象 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 扩展对象。用于懒加载某些组件。例如，评论系统并非所有客户都会使用，因此我们只在需要时才加载该扩展。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 组件的根 DOM 节点。
 * @property {string} [css]
 * @property {Object} config - FastComments 配置对象。
 * @property {Object} commentsById - 对一个按 id 存储所有评论的对象的引用，该对象会保持最新。
 * @property {Object} translations - 对所有翻译的引用。
 * @property {Function} reRenderComment - 对一个函数的引用，调用该函数可重新渲染一条评论。
 * @property {Function} removeCommentAndReRender - 对一个函数的引用，调用该函数可将评论从内存中移除并重新渲染 DOM 的相应部分。
 * @property {Function} newBroadcastId - 对一个函数的引用，调用该函数可创建新的广播 id 并将其添加到本地要忽略的广播 id 列表中。
 * @property {Function} [withSessionId] - 传入请求 URL 和回调函数进行调用。当第三方 Cookie 被阻止时（与评论和投票请求使用的路径相同），会以追加了查看者会话 id 的 URL 调用回调；否则 URL 保持不变。当无法通过该方式获取会话 id 时，会追加最后一次通过 rememberAnonSessionId 提供的匿名会话 id（如果有的话）。不用于 SSO。
 * @property {Function} [rememberAnonSessionId] - 传入响应返回的 anonSessionId 进行调用。服务器只会向请求中没有会话 Cookie 的访客返回该 id，绝不会为已登录会话返回。它保存在 localStorage 中（当存储不可用时保存在内存中），以便 withSessionId 在后续请求中发送它，从而使访客保持同一身份。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 在即将发布评论时调用。返回 false 可取消提交（例如当附加的投票未完成时）。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 过滤评论区域的 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 在渲染时过滤整个组件的 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 在渲染前过滤每条评论的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 在渲染前过滤每条评论菜单的 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 在渲染时过滤整个组件的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - （旧版）返回要添加到回复区域顶部的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - （旧版）返回要添加到组件顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - （旧版）返回要添加到评论元素顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - （旧版）返回要添加到评论元素底部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 返回要添加到评论文本之后、评论内容元素内部的 HTML（由投票功能使用）。
 * @property {Function} [replyAreaInputBottom] - 返回要添加到评论输入框内部、文本输入框下方的 HTML（由投票功能用于就地投票编辑器）。接收父评论 id，对于根回复框则为 null。
 * @property {Function} [onPollUpdate] - 当页面上某个投票的票数发生变化时，随实时事件一起调用。
 * @property {Function} isSiteAdmin - 返回查看者是否是该租户的管理员或版主。在首次获取后可知。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - （旧版）返回要添加到每条评论的菜单元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 按类名组织的点击事件处理器，可通过引用进行修改。
 * @returns void
 */

/**
 * @callback FastCommentsUIExtensionWidgetTopCallback
 * @param {Object} moduleData
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionWidgetFilter
 * @param {Object} moduleData
 * @param {Object} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionCommentTopCallback
 * @param {Object} comment
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionCommentTopFilter
 * @param {Object} comment
 * @param {string} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionCommentBottomCallback
 * @param {Object} comment
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionMenuBottomCallback
 * @param {Object} comment
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionMenuFilter
 * @param {Object} comment
 * @param {string} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionRenderCallback
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionConnectionStatusCallback
 * @param {boolean} isConnected
 * @returns {void}
 */

/**
 * @callback FastCommentsUIExtensionInitialRenderCallback
 * @returns {void}
 */

/**
 * @callback FastCommentsUIExtensionReplyAreaTop
 * @param {Object|null} currentUser
 * @param {boolean} isSaving
 * @param {boolean} isReplyOpen
 * @param {string|null} parentId
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionReplyAreaFilter
 * @param {Object|null} currentUser
 * @param {boolean} isSaving
 * @param {boolean} isReplyOpen
 * @param {string|null} parentId
 * @param {string|null} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionPrepareCommentForSavingCallback
 * @param {Object} comment
 * @param {string} parentId
 */

/**
 * @callback FastCommentsUIExtensionNewCommentCallback
 * @param {Object} comment
 */

/**
 * @callback FastCommentsUIExtensionPresenceUpdateCallback
 * @param {Object} update
 */
[inline-code-end]