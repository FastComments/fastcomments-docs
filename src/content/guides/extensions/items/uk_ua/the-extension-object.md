Об'єкт розширення складається з наступного визначення:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc об\'єкта розширення'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Об'єкт розширення FastCommentsUI. Використовується для лінивої загрузки певних компонентів. Наприклад, система відгуків
 * використовується не всіма клієнтами, тому ми завантажуємо це розширення лише тоді, коли воно потрібне.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Кореневий DOM-вузол віджета.
 * @property {string} [css]
 * @property {Object} config - Об'єкт конфігурації FastComments.
 * @property {Object} commentsById - Посилання на об'єкт з усіма коментарями за id, який постійно оновлюється.
 * @property {Object} translations - Посилання на всі переклади.
 * @property {Function} reRenderComment - Посилання на функцію, яку можна викликати для повторного рендерингу коментаря.
 * @property {Function} removeCommentAndReRender - Посилання на функцію, яку можна викликати, щоб видалити коментар з пам'яті та повторно відрендерити відповідну частину DOM.
 * @property {Function} newBroadcastId - Посилання на функцію, яку можна викликати, щоб створити новий broadcast id і додати його до локального списку broadcast id, які слід ігнорувати.
 * @property {Function} [withSessionId] - Викликається з URL запиту та callback-функцією. Викликає callback з URL, до якого додано id сесії глядача, коли сторонні cookie заблоковані (той самий шлях, який використовують запити коментарів і голосувань), або з незміненим URL в іншому випадку. Не використовується для SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Викликається з коментарем, який готується до публікації. Поверніть false, щоб скасувати відправку (наприклад, коли прикріплене опитування є неповним).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фільтрує HTML для області коментарів.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фільтрує HTML для всього віджета під час рендерингу.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фільтрує HTML для кожного коментаря перед рендерингом.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фільтрує HTML для меню кожного коментаря перед рендерингом.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фільтрує HTML для всього віджета під час рендерингу.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Повертає HTML, який додається у верхню частину області відповіді.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Повертає HTML, який додається у верхню частину віджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Повертає HTML, який додається у верхню частину елемента коментаря.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Повертає HTML, який додається у нижню частину елемента коментаря.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Повертає HTML, який додається після тексту коментаря, всередині елемента вмісту коментаря (використовується опитуваннями).
 * @property {Function} [replyAreaInputBottom] - Повертає HTML, який додається всередині рамки поля коментаря, під текстовим полем (використовується опитуваннями для вбудованого редактора опитувань). Отримує id батьківського коментаря або null для кореневого поля відповіді.
 * @property {Function} [onPollUpdate] - Викликається з live-подією, коли змінюється кількість голосів в опитуванні на сторінці.
 * @property {Function} isSiteAdmin - Повертає, чи є глядач адміністратором або модератором орендаря. Відомо після першого запиту.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Повертає HTML, який додається у нижню частину елемента меню для кожного коментаря.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Кореневий елемент.
 * @param {Object.<string, Function>} clickListeners - Обробники подій кліків, за назвою класу, які можна змінювати за посиланням.
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