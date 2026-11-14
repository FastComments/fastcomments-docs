擴充功能物件由以下定義組成：

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = '擴充功能物件 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 擴充功能物件。用於延遲載入特定元件。例如，並非所有客戶都會使用評論系統，
 * 因此我們只在需要時才載入該擴充功能。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 小工具的根 DOM 節點。
 * @property {string} [css]
 * @property {Object} config - FastComments 設定物件。
 * @property {Object} commentsById - 對包含所有按 id 索引之留言之物件的參照，並會持續保持最新。
 * @property {Object} translations - 對所有翻譯的參照。
 * @property {Function} reRenderComment - 對一個函式的參照，可呼叫該函式重新渲染一則留言。
 * @property {Function} removeCommentAndReRender - 對一個函式的參照，可呼叫該函式從記憶體中移除留言並重新渲染 DOM 的相應部分。
 * @property {Function} newBroadcastId - 對一個函式的參照，可呼叫該函式建立新的 broadcast id 並將其加入本地要忽略的 broadcast id 清單。
 * @property {Function} [withSessionId] - 傳入請求 URL 與回呼函式呼叫。當第三方 Cookie 被封鎖時（與留言及投票請求使用的路徑相同），會以附加了檢視者 session id 的 URL 呼叫回呼函式，否則 URL 保持不變。若無法透過該方式取得 session id，則附加最後一次傳給 rememberAnonSessionId 的匿名 session id（如果有的話）。不適用於 SSO。
 * @property {Function} [rememberAnonSessionId] - 傳入回應所回傳的 anonSessionId 呼叫。伺服器只會對請求中沒有 session cookie 的訪客回傳此值，且絕不會對已登入的工作階段回傳。它會保存在 localStorage 中（當儲存空間不可用時則保存在記憶體中），以便 withSessionId 在後續請求中傳送它，讓訪客保持同一個身分。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 在留言即將發佈時呼叫。回傳 false 可取消送出（例如當附加的投票尚未填寫完成時）。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 過濾留言區的 HTML。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 在渲染時過濾整個小工具的 HTML。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 在渲染前過濾每則留言的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 在渲染前過濾每則留言選單的 HTML。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 在渲染時過濾整個小工具的 HTML。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (舊版) 回傳要加入回覆區頂部的 HTML。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (舊版) 回傳要加入小工具頂部的 HTML。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (舊版) 回傳要加入留言元素頂部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (舊版) 回傳要加入留言元素底部的 HTML。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 回傳要加在留言文字之後、位於留言內容元素內的 HTML（由投票功能使用）。
 * @property {Function} [replyAreaInputBottom] - 回傳要加入留言輸入框內、位於文字輸入框下方的 HTML（投票功能用它來放置就地投票編輯器）。會接收父留言 id，根回覆框則為 null。
 * @property {Function} [onPollUpdate] - 當頁面上投票的票數變動時，會以即時事件呼叫。
 * @property {Function} isSiteAdmin - 回傳檢視者是否為該租戶的管理員或版主。在第一次請求後即可得知。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (舊版) 回傳要加入每則留言選單元素底部的 HTML。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 根元素。
 * @param {Object.<string, Function>} clickListeners - 點擊事件處理器，以類別名稱為索引，可透過參照修改。
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