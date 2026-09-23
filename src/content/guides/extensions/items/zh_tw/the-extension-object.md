擴充物件包含以下定義：

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = '擴充物件 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 擴充物件。用於延遲載入某些元件。例如，評論系統並非所有客戶都使用，因此我們僅在需要時載入該擴充功能。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 小工具根 DOM 節點。
 * @property {string} [css]
 * @property {Object} config - FastComments 設定物件。
 * @property {Object} commentsById - 指向一個包含所有依 ID 分類之評論的物件參考，會保持最新。
 * @property {Object} translations - 指向所有翻譯的參考。
 * @property {Function} reRenderComment - 可呼叫以重新渲染評論的函式參考。
 * @property {Function} removeCommentAndReRender - 可呼叫以從記憶體中移除評論並重新渲染 DOM 中相應部分的函式參考。
 * @property {Function} newBroadcastId - 可呼叫以建立新廣播 ID 並將其加入本地要忽略的廣播 ID 清單的函式參考。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 在即將發佈的評論上被呼叫。返回 false 可取消提交（例如當附加的投票未完成時）。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 過濾評論區域的 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 在渲染時過濾整個小工具的 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 在渲染前過濾每則評論的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 在渲染前過濾每則評論選單的 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 在渲染時過濾整個小工具的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) (舊版) 回傳要加入回覆區域頂部的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) (舊版) 回傳要加入小工具頂部的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) (舊版) 回傳要加入評論元素頂部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) (舊版) 回傳要加入評論元素底部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 回傳要在評論文字之後、評論內容元素內加入的 HTML（用於投票）。
 * @property {Function} [replyAreaInputBottom] - 回傳要在評論輸入框內、文字輸入下方加入的 HTML（用於投票的即時投票編輯器）。接收父評論 ID，若為根回覆框則為 null。
 * @property {Function} [onPollUpdate] - 在頁面上投票計數變更時，透過即時事件呼叫。
 * @property {Function} isSiteAdmin - 回傳檢視者是否為租戶的管理員或版主。於首次取得後得知。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) (舊版) 回傳要加入每則評論選單元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 依類別名稱的點擊事件處理函式，可透過參考進行修改。
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