---
扩展对象包含以下定义：

<!-- 如果要更新此内容，请记得更新 comment-ui-core -->
[inline-code-attrs-start title = '扩展对象 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 小部件的根 DOM 节点。
 * @property {string} [css]
 * @property {Object} config - FastComments 配置对象。
 * @property {Object} commentsById - 对按 id 存储的所有评论的引用，并实时更新。
 * @property {Object} translations - 对所有翻译的引用。
 * @property {Function} reRenderComment - 可调用以重新渲染评论的函数引用。
 * @property {Function} removeCommentAndReRender - 可调用以从内存中移除评论并重新渲染相应 DOM 部分的函数引用。
 * @property {Function} newBroadcastId - 可调用以创建新的广播 id 并将其添加到本地忽略的广播 id 列表中的函数引用。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 过滤回复区域的 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 在渲染时过滤整个小部件的 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 在渲染前为每条评论过滤 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 在渲染前为每条评论的菜单过滤 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 在渲染时为整个小部件过滤 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) 返回要添加到回复区域顶部的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) 返回要添加到小部件顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) 返回要添加到评论元素顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) 返回要添加到评论元素底部的 HTML。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) 返回要添加到每条评论的菜单元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 按类名组织的点击事件处理程序，可通过引用进行修改。
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


---