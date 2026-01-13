Обектът на разширението се състои от следното определение:

<!-- ако искате да актуализирате това, не забравяйте да актуализирате comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc на обекта Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Коренният DOM възел на уиджета.
 * @property {string} [css]
 * @property {Object} config - Конфигурационният обект на FastComments.
 * @property {Object} commentsById - Референция към обект с всички коментари по id, който се поддържа актуален.
 * @property {Object} translations - Референция към всички преводи.
 * @property {Function} reRenderComment - Референция към функция, която може да бъде извикана за повторно рендиране на коментар.
 * @property {Function} removeCommentAndReRender - Референция към функция, която може да бъде извикана, за да премахне коментар от паметта и да повторно рендира съответната част от DOM-а.
 * @property {Function} newBroadcastId - Референция към функция, която може да бъде извикана, за да създаде нов broadcast id и да го добави към локалния списък с broadcast id-та за игнориране.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Филтър за HTML на областта за коментари.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Филтър за HTML на целия уиджет при рендиране.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Филтър за HTML за всеки коментар преди рендиране.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Филтър за HTML за всяко меню на коментар преди рендиране.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Филтър за HTML на целия уиджет при рендиране.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Връща HTML за добавяне в горната част на зоната за отговори.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Връща HTML за добавяне в горната част на уиджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Връща HTML за добавяне в горната част на елемента на коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Връща HTML за добавяне в долната част на елемента на коментара.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Връща HTML за добавяне в долната част на елемента на менюто за всеки коментар.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Коренният елемент.
 * @param {Object.<string, Function>} clickListeners - Обработчиците на събития за кликове, индексирани по име на клас, които могат да бъдат модифицирани чрез референция.
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