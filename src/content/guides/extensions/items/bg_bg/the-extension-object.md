The extension object consists of the following definition:

<!-- ако искате да актуализирате това, не забравяйте да актуализирате comment-ui-core -->
[inline-code-attrs-start title = 'Обект на разширението JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI разширителният обект. Използва се за lazy-loading (мързеливо зареждане) на определени компоненти. Например, системата за отзиви не
 * се използва от всички клиенти, затова зареждаме това разширение само когато ни е необходимо.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Кореновият DOM елемент на уиджета.
 * @property {string} [css]
 * @property {Object} config - Конфигурационният обект на FastComments.
 * @property {Object} commentsById - Препратка към обект, съдържащ всички коментари по ID, който се поддържа актуален.
 * @property {Object} translations - Препратка към всички преводи.
 * @property {Function} reRenderComment - Препратка към функция, която може да се извика за повторно рендериране на коментар.
 * @property {Function} removeCommentAndReRender - Препратка към функция, която може да се извика за премахване на коментар от паметта и повторно рендериране на съответната част от DOM.
 * @property {Function} newBroadcastId - Препратка към функция, която може да се извика за създаване на нов broadcast ID и добавянето му към локалния списък с broadcast ID‑та за игнориране.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Извиква се с коментара, който предстои да бъде публикуван. Върнете false, за да отмените изпращането (например когато прикаченото гласуване е непълно).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Филтрира HTML за областта на коментара.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Филтрира HTML за целия уиджет при рендериране.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Филтрира HTML за всеки коментар преди рендериране.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Филтрира HTML за всяко меню на коментар преди рендериране.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Филтрира HTML за целия уиджет при рендериране.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Връща HTML, който се добавя в горната част на областта за отговор.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Връща HTML, който се добавя в горната част на уиджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Връща HTML, който се добавя в горната част на елемента на коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Връща HTML, който се добавя в долната част на елемента на коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Връща HTML, който се добавя след текста на коментара, вътре в елемента за съдържание на коментара (използва се от анкети).
 * @property {Function} [replyAreaInputBottom] - Връща HTML, който се добавя вътре в рамката за въвеждане на коментар, под текстовото поле (използва се от анкети за вграден редактор на анкети). Приема ID на родителския коментар или null за кореновото поле за отговор.
 * @property {Function} [onPollUpdate] - Извиква се с живото събитие, когато броят на гласовете в анкета на страницата се промени.
 * @property {Function} isSiteAdmin - Връща дали зрителят е администратор или модератор на наемателя. Известно след първото извличане.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Връща HTML, който се добавя в долната част на елемента за меню за всеки коментар.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Кореновият елемент.
 * @param {Object.<string, Function>} clickListeners - Обработчиците на събития за кликвания, по име на клас, които могат да се модифицират по референция.
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