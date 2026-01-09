Об'єкт розширення складається з наступного визначення:

<!-- якщо ви хочете оновити це, не забудьте також оновити comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc об'єкта розширення'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Об'єкт розширення FastCommentsUI. Використовується для відкладеного (lazy) завантаження певних компонентів. Наприклад, система рецензій
 * не використовується всіма клієнтами, тому ми завантажуємо це розширення лише тоді, коли воно потрібно.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Кореневий DOM-вузол віджета.
 * @property {string} [css]
 * @property {Object} config - Об'єкт конфігурації FastComments.
 * @property {Object} commentsById - Посилання на об'єкт з усіма коментарями за id, який підтримується в актуальному стані.
 * @property {Object} translations - Посилання на всі переклади.
 * @property {Function} reRenderComment - Посилання на функцію, яку можна викликати для повторного рендерингу коментаря.
 * @property {Function} removeCommentAndReRender - Посилання на функцію, яку можна викликати для видалення коментаря з пам'яті та повторного рендерингу відповідної частини DOM.
 * @property {Function} newBroadcastId - Посилання на функцію, яку можна викликати для створення нового broadcast id та додавання його до локального списку broadcast id, які слід ігнорувати.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фільтрує HTML для області відповіді на коментар.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фільтрує HTML для всього віджета під час рендерингу.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фільтрує HTML для кожного коментаря перед рендерингом.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фільтрує HTML для меню кожного коментаря перед рендерингом.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фільтрує HTML для всього віджета під час рендерингу.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ЗАСТАРІЛЕ) Повертає HTML, який додається до верхньої частини області відповіді.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ЗАСТАРІЛЕ) Повертає HTML, який додається до верхньої частини віджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ЗАСТАРІЛЕ) Повертає HTML, який додається до верхньої частини елемента коментаря.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ЗАСТАРІЛЕ) Повертає HTML, який додається до нижньої частини елемента коментаря.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ЗАСТАРІЛЕ) Повертає HTML, який додається до нижньої частини елемента меню для кожного коментаря.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Кореневий елемент.
 * @param {Object.<string, Function>} clickListeners - Обробники подій для кліків, згруповані за іменем класу, які можна змінювати за посиланням.
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