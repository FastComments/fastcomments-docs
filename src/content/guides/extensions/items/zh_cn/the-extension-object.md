扩展对象由以下定义组成：

<!-- 如果您想更新此内容，请记得更新 comment-ui-core -->
[inline-code-attrs-start title = '扩展对象 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 扩展对象。用于惰性加载某些组件。例如，评论系统并非所有客户都使用，因此我们仅在需要时加载该扩展。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 小部件根 DOM 节点。
 * @property {string} [css]
 * @property {Object} config - FastComments 配置对象。
 * @property {Object} commentsById - 指向一个按 ID 存放所有评论的对象的引用，该对象会保持最新。
 * @property {Object} translations - 指向所有翻译的引用。
 * @property {Function} reRenderComment - 指向一个可调用以重新渲染评论的函数的引用。
 * @property {Function} removeCommentAndReRender - 指向一个可调用以从内存中移除评论并重新渲染 DOM 相应部分的函数的引用。
 * @property {Function} newBroadcastId - 指向一个可调用以创建新广播 ID 并将其添加到本地要忽略的广播 ID 列表的函数的引用。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 在即将发布的评论上调用。返回 false 可取消提交（例如当附带的投票未完成时）。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 过滤评论区域的 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 在渲染时过滤整个小部件的 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 在渲染前过滤每条评论的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 在渲染前过滤每个评论菜单的 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 在渲染时过滤整个小部件的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) 返回要添加到回复区域顶部的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) 返回要添加到小部件顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) 返回要添加到评论元素顶部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) 返回要添加到评论元素底部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 返回要在评论文本之后、评论内容元素内部添加的 HTML（用于投票）。
 * @property {Function} [replyAreaInputBottom] - 返回要在评论输入框内部、文本输入下方添加的 HTML（用于投票的内嵌投票编辑器）。接收父评论 ID，根回复框时为 null。
 * @property {Function} [onPollUpdate] - 在页面上投票计数变化时，以实时事件调用。
 * @property {Function} isSiteAdmin - 返回查看者是否为租户的管理员或版主。首次获取后可知。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) 返回要添加到每条评论的菜单元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 按类名划分的点击事件处理函数，可通过引用进行修改。
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