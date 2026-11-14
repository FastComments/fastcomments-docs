扩展对象由以下定义组成：

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = '扩展对象 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 扩展对象。用于懒加载某些组件。例如，评论系统并非所有客户都会使用，
 * 因此我们只在需要时才加载该扩展。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 组件的根 DOM 节点。
 * @property {string} [css]
 * @property {Object} config - FastComments 配置对象。
 * @property {Object} commentsById - 对一个包含所有按 id 索引的评论的对象的引用，该对象会保持最新。
 * @property {Object} translations - 对所有翻译的引用。
 * @property {Function} reRenderComment - 对一个可调用来重新渲染评论的函数的引用。
 * @property {Function} removeCommentAndReRender - 对一个可调用来从内存中移除评论并重新渲染 DOM 相应部分的函数的引用。
 * @property {Function} newBroadcastId - 对一个可调用来创建新的广播 id 并将其添加到本地要忽略的广播 id 列表的函数的引用。
 * @property {Function} [withSessionId] - 传入请求 URL 和回调函数进行调用。当第三方 Cookie 被阻止时，回调会收到追加了查看者会话 id 的 URL（与评论和投票请求使用的路径相同），否则 URL 保持不变。不用于 SSO。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 在即将发布的评论被调用。返回 false 可取消提交（例如当附加的投票未完成时）。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 过滤评论区域的 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 渲染时过滤整个组件的 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 渲染前过滤每条评论的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 渲染前过滤每条评论菜单的 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 渲染时过滤整个组件的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - （旧版）返回要添加到回复区域顶部的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - （旧版）返回要添加到组件顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - （旧版）返回要添加到评论元素顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - （旧版）返回要添加到评论元素底部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 返回要添加到评论文本之后、评论内容元素内部的 HTML（投票功能使用）。
 * @property {Function} [replyAreaInputBottom] - 返回要添加到评论输入框内部、文本输入框下方的 HTML（投票功能用于就地投票编辑器）。接收父评论 id，根回复框则为 null。
 * @property {Function} [onPollUpdate] - 当页面上某个投票的票数变化时，随实时事件被调用。
 * @property {Function} isSiteAdmin - 返回查看者是否是该租户的管理员或版主。在首次请求后可知。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - （旧版）返回要添加到每条评论的菜单元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 按类名索引的点击事件处理器，可通过引用进行修改。
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