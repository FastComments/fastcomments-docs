Об’єкт розширення складається з наступного визначення:

<!-- якщо ви хочете оновити це, пам’ятайте оновити comment-ui-core -->
[inline-code-attrs-start title = 'Об’єкт розширення JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Об’єкт розширення FastCommentsUI. Використовується для відкладеного завантаження певних компонентів. 
 * Наприклад, система відгуків не використовується всіма клієнтами, тому ми завантажуємо це розширення лише коли це потрібно.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Кореневий DOM‑елемент віджета.
 * @property {string} [css]
 * @property {Object} config - Об’єкт конфігурації FastComments.
 * @property {Object} commentsById - Посилання на об’єкт з усіма коментарями за ідентифікатором, який постійно оновлюється.
 * @property {Object} translations - Посилання на всі переклади.
 * @property {Function} reRenderComment - Посилання на функцію, яку можна викликати для повторного рендерингу коментаря.
 * @property {Function} removeCommentAndReRender - Посилання на функцію, яку можна викликати для видалення коментаря з пам’яті та повторного рендерингу відповідної частини DOM.
 * @property {Function} newBroadcastId - Посилання на функцію, яку можна викликати для створення нового broadcast‑id та додавання його до локального списку broadcast‑id, які ігноруються.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Викликається з коментарем, який збираються опублікувати. Повернути false, щоб скасувати відправку (наприклад, коли прикріплений опитування неповний).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Фільтрує HTML для області коментаря.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Фільтрує HTML для всього віджета під час рендерингу.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Фільтрує HTML для кожного коментаря перед рендерингом.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Фільтрує HTML для кожного меню коментаря перед рендерингом.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Фільтрує HTML для всього віджета під час рендерингу.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Повертає HTML, який додається у верхню частину області відповіді.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Повертає HTML, який додається у верхню частину віджета.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Повертає HTML, який додається у верхню частину елементу коментаря.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Повертає HTML, який додається у нижню частину елементу коментаря.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Повертає HTML, який додається після тексту коментаря, всередині елементу вмісту коментаря (використовується в опитуваннях).
 * @property {Function} [replyAreaInputBottom] - Повертає HTML, який додається всередині рамки вводу коментаря, під текстовим полем (використовується в опитуваннях для вбудованого редактора опитувань). Приймає ідентифікатор батьківського коментаря або null для кореневої області відповіді.
 * @property {Function} [onPollUpdate] - Викликається під час живої події, коли змінюються підрахунки голосів в опитуванні на сторінці.
 * @property {Function} isSiteAdmin - Повертає, чи є переглядач адміністратором або модератором орендаря. Відомо після першого запиту.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Повертає HTML, який додається у нижню частину елементу меню для кожного коментаря.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @ {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @ {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @ {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Кореневий елемент.
 * @param {Object.<string, Function>} clickListeners - Обробники подій кліків за назвою класу, які можуть бути змінені за посиланням.
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