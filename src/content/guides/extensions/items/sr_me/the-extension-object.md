The extension object consists of the following definition:

<!-- ако желите да ажурирате ово, запамтите да ажурирате comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc објекта екстензије'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Коренски DOM чвор виџета.
 * @property {string} [css]
 * @property {Object} config - FastComments конфигурациони објекат.
 * @property {Object} commentsById - Референца на објекат са свим коментарима по id, која се одржава ажурном.
 * @property {Object} translations - Референца на све преводе.
 * @property {Function} reRenderComment - Референца на функцију која се може позвати да поново рендерује коментар.
 * @property {Function} removeCommentAndReRender - Референца на функцију која се може позвати да уклони коментар из меморије и поново рендерује одговарајући део DOM-а.
 * @property {Function} newBroadcastId - Референца на функцију која се може позвати да креира нови broadcast id и дода га у локалну листу broadcast id-ева које треба игнорисати.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Филтрира HTML за област коментара.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Филтрира HTML за цео виџет при рендеровању.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Филтрира HTML за сваки коментар пре рендера.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Филтрира HTML за мени сваког коментара пре рендера.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Филтрира HTML за цео виџет при рендеровању.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ЗАСТАРЕЛО) Враћа HTML који ће се додати на врх области за одговор.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ЗАСТАРЕЛО) Враћа HTML који ће се додати на врх виџета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ЗАСТАРЕЛО) Враћа HTML који ће се додати на врх елемента коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ЗАСТАРЕЛО) Враћа HTML који ће се додати на дно елемента коментара.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ЗАСТАРЕЛО) Враћа HTML који ће се додати на дно елемента менија за сваки коментар.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Коренски елемент.
 * @param {Object.<string, Function>} clickListeners - Обработачи догађаја за кликове, по имену класе, који се могу изменити преко референце.
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