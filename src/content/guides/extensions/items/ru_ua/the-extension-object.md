The extension object consists of the following definition:

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
 * @property {Function} newBroadcastId - Ссылка на функцию, которую можно вызвать, чтобы создать новый broadcast id и добавить его в локальный список broadcast id для игнорирования.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фильтрация HTML для области комментариев.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фильтрация HTML для всего виджета при рендере.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фильтрация HTML для каждого комментария перед рендером.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фильтрация HTML для меню каждого комментария перед рендером.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фильтрация HTML для всего виджета при рендере.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Возвращает HTML для добавления в верхнюю часть области ответа.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Возвращает HTML для добавления в верхнюю часть виджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Возвращает HTML для добавления в верхнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Возвращает HTML для добавления в нижнюю часть элемента комментария.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Возвращает HTML для добавления в нижнюю часть элемента меню для каждого комментария.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Корневой элемент.
 * @param {Object.<string, Function>} clickListeners - Обработчики кликов, с ключом по имени класса, которые можно изменить по ссылке.
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