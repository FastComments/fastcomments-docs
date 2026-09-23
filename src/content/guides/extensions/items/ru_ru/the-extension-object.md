Объект расширения состоит из следующего определения:

<!-- если вы хотите обновить это, не забудьте обновить comment-ui-core -->
[inline-code-attrs-start title = 'Объект расширения JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Объект расширения FastCommentsUI. Используется для отложенной загрузки некоторых компонентов. Например, система отзывов не
 * используется всеми клиентами, поэтому мы загружаем это расширение только когда это необходимо.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Корневой DOM‑узел виджета.
 * @property {string} [css]
 * @property {Object} config - Объект конфигурации FastComments.
 * @property {Object} commentsById - Ссылка на объект со всеми комментариями по id, который поддерживается в актуальном состоянии.
 * @property {Object} translations - Ссылка на все переводы.
 * @property {Function} reRenderComment - Ссылка на функцию, которую можно вызвать для повторного рендеринга комментария.
 * @property {Function} removeCommentAndReRender - Ссылка на функцию, которую можно вызвать для удаления комментария из памяти и повторного рендеринга соответствующей части DOM.
 * @property {Function} newBroadcastId - Ссылка на функцию, которую можно вызвать для создания нового broadcast‑id и добавления его в локальный список broadcast‑id, которые следует игнорировать.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Вызывается с комментарием, который собираются опубликовать. Верните false, чтобы отменить отправку (например, когда прикреплённый опрос не завершён).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фильтровать HTML для области комментариев.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фильтровать HTML для всего виджета при рендере.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фильтровать HTML для каждого комментария перед рендером.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фильтровать HTML для меню каждого комментария перед рендером.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фильтровать HTML для всего виджета при рендере.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (УСТАРЕВШЕ) Возвращает HTML, который будет добавлен в верхнюю часть области ответа.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (УСТАРЕВШЕ) Возвращает HTML, который будет добавлен в верхнюю часть виджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (УСТАРЕВШЕ) Возвращает HTML, который будет добавлен в верхнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (УСТАРЕВШЕ) Возвращает HTML, который будет добавлен в нижнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Возвращает HTML, который будет добавлен после текста комментария, внутри элемента содержимого комментария (используется в опросах).
 * @property {Function} [replyAreaInputBottom] - Возвращает HTML, который будет добавлен внутри рамки ввода комментария, под текстовым полем (используется в опросах для встроенного редактора опросов). Принимает id родительского комментария или null для корневого поля ответа.
 * @property {Function} [onPollUpdate] - Вызывается с событием в реальном времени, когда меняется количество голосов в опросе на странице.
 * @property {Function} isSiteAdmin - Возвращает, является ли пользователь администратором или модератором арендатора. Известно после первого запроса.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (УСТАРЕВШЕ) Возвращает HTML, который будет добавлен в нижнюю часть элемента меню для каждого комментария.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Корневой элемент.
 * @param {Object.<string, Function>} clickListeners - Обработчики событий кликов, по имени класса, которые могут быть изменены по ссылке.
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