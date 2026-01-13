擴充套件物件由下列定義組成：

<!-- 如果您想要更新這個，請記得更新 comment-ui-core -->
[inline-code-attrs-start title = '擴充套件物件 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - widget 的根 DOM 節點。
 * @property {string} [css]
 * @property {Object} config - FastComments 的設定物件。
 * @property {Object} commentsById - 參考一個以 id 為鍵、包含所有評論的物件，並會持續保持更新。
 * @property {Object} translations - 參考所有翻譯資料。
 * @property {Function} reRenderComment - 參考一個可用來重新渲染評論的函式。
 * @property {Function} removeCommentAndReRender - 參考一個可用來從記憶體中移除評論並重新渲染對應 DOM 區段的函式。
 * @property {Function} newBroadcastId - 參考一個可用來建立新的廣播 id 並將其加入本地要忽略的廣播 id 清單的函式。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 為評論區過濾 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 在渲染時為整個 widget 過濾 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 在渲染前為每個評論過濾 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 在渲染前為每則評論的選單過濾 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 在渲染時為整個 widget 過濾 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - （遺留）回傳要新增至回覆區頂端的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - （遺留）回傳要新增至 widget 頂端的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - （遺留）回傳要新增至評論元素頂端的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - （遺留）回傳要新增至評論元素底部的 HTML。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - （遺留）回傳要新增至每則評論的選單元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 以 class 名稱為鍵的點擊事件處理器，可透過參考來修改。
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