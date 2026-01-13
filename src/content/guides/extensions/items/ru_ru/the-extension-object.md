Объект расширения состоит из следующего определения:

<!-- если вы хотите обновить это, не забудьте обновить comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc объекта расширения'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Корневой DOM-узел виджета.
 * @property {string} [css]
 * @property {Object} config - Объект конфигурации FastComments.
 * @property {Object} commentsById - Ссылка на объект со всеми комментариями по id, который поддерживается в актуальном состоянии.
 * @property {Object} translations - Ссылка на все переводы.
 * @property {Function} reRenderComment - Ссылка на функцию, которую можно вызвать для повторного рендеринга комментария.
 * @property {Function} removeCommentAndReRender - Ссылка на функцию, которую можно вызвать, чтобы удалить комментарий из памяти и повторно отрендерить соответствующую часть DOM.
 * @property {Function} newBroadcastId - Ссылка на функцию, которую можно вызвать для создания нового broadcast id и добавления его в локальный список broadcast id для игнорирования.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фильтр HTML для области комментариев.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фильтр HTML для всего виджета при рендеринге.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фильтр HTML для каждого комментария перед рендерингом.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фильтр HTML для меню каждого комментария перед рендерингом.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фильтр HTML для всего виджета при рендеринге.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в верхнюю часть области ответа.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в верхнюю часть виджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в верхнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в нижнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (УСТАРЕВШЕЕ) Возвращает HTML для добавления в нижнюю часть элемента меню для каждого комментария.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Корневой элемент.
 * @param {Object.<string, Function>} clickListeners - Обработчики событий для кликов, по имени класса, которые могут быть изменены по ссылке.
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