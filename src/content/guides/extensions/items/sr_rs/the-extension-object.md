<!-- ако желите да ажурирате ово, запамтите да ажурирате comment-ui-core -->
[inline-code-attrs-start title = 'Објекат проширења JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI објекат проширења. Користи се за лениво учитавање одређених компоненти. На пример,
 * систем за рецензије не користе сви клијенти, па учитавамо то проширење само када нам је потребно.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - DOM чвор корена виџета.
 * @property {string} [css]
 * @property {Object} config - FastComments конфигурациони објекат.
 * @property {Object} commentsById - Референца на објекат са свим коментарима по ID-у, који се одржава ажурним.
 * @property {Object} translations - Референца на све преводе.
 * @property {Function} reRenderComment - Референца на функцију која се може позвати за поновно рендеровање коментара.
 * @property {Function} removeCommentAndReRender - Референца на функцију која се може позвати за уклањање коментара из меморије и поновно рендеровање одговарајућег дела DOM-а.
 * @property {Function} newBroadcastId - Референца на функцију која се може позвати за креирање новог broadcast ID-а и додавање у локални списак broadcast ID-ова за игнорисање.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Позива се са коментаром који ће бити објављен. Врати false да отказеш слање (на пример када је прикачена анкета непотпуна).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Филтрира HTML за област коментара.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Филтрира HTML за цео виџет приликом рендеровања.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Филтрира HTML за сваки коментар пре рендеровања.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Филтрира HTML за сваки мени коментара пре рендеровања.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Филтрира HTML за цео виџет приликом рендеровања.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Враћа HTML који се додаје на врх области одговора.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Враћа HTML који се додаје на врх виџета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Враћа HTML који се додаје на врх елемента коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Враћа HTML који се додаје на дно елемента коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Враћа HTML који се додаје након текста коментара, унутар елемента садржаја коментара (користи се за анкете).
 * @property {Function} [replyAreaInputBottom] - Враћа HTML који се додаје унутар оквира уноса коментара, испод текстуалног уноса (користи се за анкете у уређивачу на месту). Прихвата ID родитељског коментара, или null за оквир коренског одговора.
 * @property {Function} [onPollUpdate] - Позива се са живим догађајем када се промене бројеви гласова у анкети на страници.
 * @property {Function} isSiteAdmin - Враћа да ли је посматрач администратор или модератор tenancy-а. Познато након првог преузимања.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Враћа HTML који се додаје на дно елемента менија за сваки коментар.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - корени елемент.
 * @param {Object.<string, Function>} clickListeners - Хендлери догађаја за кликове, по имену класе, који се могу модификовати по референци.
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