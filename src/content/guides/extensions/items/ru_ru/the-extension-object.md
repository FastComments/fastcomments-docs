Объект расширения состоит из следующего определения:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc объекта расширения'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Объект расширения FastCommentsUI. Используется для ленивой загрузки определённых компонентов. Например, система отзывов
 * используется не всеми клиентами, поэтому мы загружаем это расширение только тогда, когда оно нужно.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Корневой DOM-узел виджета.
 * @property {string} [css]
 * @property {Object} config - Объект конфигурации FastComments.
 * @property {Object} commentsById - Ссылка на объект со всеми комментариями по id, который постоянно обновляется.
 * @property {Object} translations - Ссылка на все переводы.
 * @property {Function} reRenderComment - Ссылка на функцию, которую можно вызвать для повторного рендера комментария.
 * @property {Function} removeCommentAndReRender - Ссылка на функцию, которую можно вызвать для удаления комментария из памяти и повторного рендера соответствующей части DOM.
 * @property {Function} newBroadcastId - Ссылка на функцию, которую можно вызвать для создания нового broadcast id и добавления его в локальный список broadcast id для игнорирования.
 * @property {Function} [withSessionId] - Вызывается с URL запроса и колбэком. Вызывает колбэк с URL, к которому добавлен id сессии зрителя, когда сторонние cookie заблокированы (тот же путь, что используют запросы комментариев и голосования), либо с неизменённым URL в противном случае. Не используется для SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Вызывается с комментарием, который собирается быть отправлен. Верните false, чтобы отменить отправку (например, когда прикреплённый опрос не заполнен).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фильтр HTML для области комментариев.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фильтр HTML для всего виджета при рендере.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фильтр HTML для каждого комментария перед рендером.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фильтр HTML для меню каждого комментария перед рендером.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фильтр HTML для всего виджета при рендере.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в верхнюю часть области ответов.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в верхнюю часть виджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в верхнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в нижнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Возвращает HTML для добавления после текста комментария, внутри элемента содержимого комментария (используется опросами).
 * @property {Function} [replyAreaInputBottom] - Возвращает HTML для добавления внутри рамки ввода комментария, под текстовым полем (используется опросами для встроенного редактора опроса). Получает id родительского комментария или null для корневого поля ответа.
 * @property {Function} [onPollUpdate] - Вызывается с событием в реальном времени, когда изменяются счётчики голосов опроса на странице.
 * @property {Function} isSiteAdmin - Возвращает, является ли зритель администратором или модератором тенанта. Известно после первого запроса данных.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в нижнюю часть элемента меню для каждого комментария.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Корневой элемент.
 * @param {Object.<string, Function>} clickListeners - Обработчики событий кликов, по имени класса, которые можно изменять по ссылке.
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