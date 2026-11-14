Обектът extension се състои от следната дефиниция:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc на обекта Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Обектът за разширение на FastCommentsUI. Използва се за мързеливо зареждане на определени компоненти. Например системата за ревюта не се
 * използва от всички клиенти, така че зареждаме това разширение само когато го искаме.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Основният DOM възел на уиджета.
 * @property {string} [css]
 * @property {Object} config - Конфигурационният обект на FastComments.
 * @property {Object} commentsById - Референция към обект с всички коментари по id, който се поддържа актуален.
 * @property {Object} translations - Референция към всички преводи.
 * @property {Function} reRenderComment - Референция към функция, която може да бъде извикана за повторно изобразяване на коментар.
 * @property {Function} removeCommentAndReRender - Референция към функция, която може да бъде извикана за премахване на коментар от паметта и повторно изобразяване на съответната част от DOM.
 * @property {Function} newBroadcastId - Референция към функция, която може да бъде извикана за създаване на нов broadcast id и добавянето му към локалния списък с broadcast ids, които да бъдат игнорирани.
 * @property {Function} [withSessionId] - Извиква се с URL на заявка и callback. Извиква callback-а с URL-а плюс session id на зрителя, добавен когато бисквитките на трети страни са блокирани (същият път, който използват заявките за коментари и гласуване), или URL-а непроменен в противен случай. Когато няма наличен session id по този начин, добавя последния анонимен session id, подаден на rememberAnonSessionId, ако има такъв. Не се използва за SSO.
 * @property {Function} [rememberAnonSessionId] - Извиква се с anonSessionId, върнат от отговор. Сървърът го връща само на гост, чиято заявка няма session бисквитка, и никога за влязла сесия. Пази се в localStorage (или в паметта, когато хранилището не е налично), така че withSessionId го изпраща при следващи заявки и гостът запазва една идентичност.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Извиква се с коментара, който предстои да бъде публикуван. Върнете false, за да отмените изпращането (например когато прикачена анкета е непълна).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Филтрира HTML за зоната с коментари.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Филтрира HTML за целия уиджет при изобразяване.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Филтрира HTML за всеки коментар преди изобразяване.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Филтрира HTML за менюто на всеки коментар преди изобразяване.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Филтрира HTML за целия уиджет при изобразяване.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Връща HTML, който да бъде добавен в горната част на зоната за отговор.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Връща HTML, който да бъде добавен в горната част на уиджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Връща HTML, който да бъде добавен в горната част на елемента с коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Връща HTML, който да бъде добавен в долната част на елемента с коментара.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Връща HTML, който да бъде добавен след текста на коментара, вътре в елемента със съдържанието на коментара (използва се от анкети).
 * @property {Function} [replyAreaInputBottom] - Връща HTML, който да бъде добавен вътре в рамката на полето за коментар, под текстовото поле (използва се от анкети за редактора на анкетата на място). Получава id на родителския коментар или null за основното поле за отговор.
 * @property {Function} [onPollUpdate] - Извиква се с live събитието, когато броячите на гласове на анкета на страницата се променят.
 * @property {Function} isSiteAdmin - Връща дали зрителят е администратор или модератор на наемателя. Известно след първото извличане.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Връща HTML, който да бъде добавен в долната част на елемента с менюто за всеки коментар.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Основният елемент.
 * @param {Object.<string, Function>} clickListeners - Обработчиците на събития за кликвания, по име на клас, които могат да бъдат променяни по референция.
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