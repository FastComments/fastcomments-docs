Објекат проширења се састоји од следеће дефиниције:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc документација објекта проширења'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Објекат проширења FastCommentsUI. Користи се за лењо учитавање одређених компоненти. На пример, систем рецензија не
 * користе сви корисници, па учитавамо то проширење само када нам затреба.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Коренски DOM чвор виџета.
 * @property {string} [css]
 * @property {Object} config - Конфигурациони објекат FastComments.
 * @property {Object} commentsById - Референца на објекат са свим коментарима по id, који се стално ажурира.
 * @property {Object} translations - Референца на све преводе.
 * @property {Function} reRenderComment - Референца на функцију која се може позвати да поново прикаже коментар.
 * @property {Function} removeCommentAndReRender - Референца на функцију која се може позвати да уклони коментар из меморије и поново прикаже одговарајући део DOM-а.
 * @property {Function} newBroadcastId - Референца на функцију која се може позвати да креира нови broadcast id и дода га у локалну листу broadcast id-јева за игнорисање.
 * @property {Function} [withSessionId] - Позива се са URL-ом захтева и callback функцијом. Позива callback са URL-ом коме је додат session id гледаоца када су third-party колачићи блокирани (исти пут који користе захтеви за коментаре и гласање), или са непромењеним URL-ом у супротном. Када session id није доступан на тај начин, додаје анонимни session id последњи прослеђен функцији rememberAnonSessionId, ако постоји. Не користи се за SSO.
 * @property {Function} [rememberAnonSessionId] - Позива се са anonSessionId који је вратио одговор. Сервер га враћа само госту чији захтев није имао колачић сесије, и никада за пријављену сесију. Чува се у localStorage (или у меморији када складиште није доступно) тако да withSessionId шаље у каснијим захтевима и да гост задржи један идентитет.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Позива се са коментаром који ће бити објављен. Вратите false да бисте отказали слање (на пример када је приложена анкета непотпуна).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Филтрира HTML за област коментара.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Филтрира HTML за цео виџет при приказу.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Филтрира HTML за сваки коментар пре приказа.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Филтрира HTML за мени сваког коментара пре приказа.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Филтрира HTML за цео виџет при приказу.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ЗАСТАРЕЛО) Враћа HTML који се додаје на врх области за одговор.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ЗАСТАРЕЛО) Враћа HTML који се додаје на врх виџета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ЗАСТАРЕЛО) Враћа HTML који се додаје на врх елемента коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ЗАСТАРЕЛО) Враћа HTML који се додаје на дно елемента коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Враћа HTML који се додаје после текста коментара, унутар елемента садржаја коментара (користе анкете).
 * @property {Function} [replyAreaInputBottom] - Враћа HTML који се додаје унутар оквира за унос коментара, испод поља за текст (користе анкете за уређивач анкете на месту). Прима id родитељског коментара, или null за коренско поље за одговор.
 * @property {Function} [onPollUpdate] - Позива се са live догађајем када се промене бројачи гласова анкете на страници.
 * @property {Function} isSiteAdmin - Враћа да ли је гледалац администратор или модератор тенанта. Постаје познато после првог дохвата.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ЗАСТАРЕЛО) Враћа HTML који се додаје на дно елемента менија за сваки коментар.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Коренски елемент.
 * @param {Object.<string, Function>} clickListeners - Обрађивачи догађаја за кликове, по називу класе, који се могу мењати по референци.
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