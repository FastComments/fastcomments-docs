Obiekt rozszerzenia składa się z następującej definicji:

<!-- jeśli chcesz to zaktualizować, pamiętaj o zaktualizowaniu comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc obiektu rozszerzenia'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Główny węzeł DOM widżetu.
 * @property {string} [css]
 * @property {Object} config - Obiekt konfiguracyjny FastComments.
 * @property {Object} commentsById - Odwołanie do obiektu ze wszystkimi komentarzami według id, które jest na bieżąco aktualizowane.
 * @property {Object} translations - Odwołanie do wszystkich tłumaczeń.
 * @property {Function} reRenderComment - Odwołanie do funkcji, którą można wywołać, aby ponownie wyrenderować komentarz.
 * @property {Function} removeCommentAndReRender - Odwołanie do funkcji, którą można wywołać, aby usunąć komentarz z pamięci i ponownie wyrenderować odpowiednią część DOM.
 * @property {Function} newBroadcastId - Odwołanie do funkcji, którą można wywołać, aby utworzyć nowe id broadcast i dodać je do lokalnej listy id broadcast do zignorowania.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtr HTML dla obszaru odpowiedzi.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtr HTML dla całego widżetu podczas renderowania.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtr HTML dla każdego komentarza przed renderowaniem.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtr HTML dla każdego menu komentarza przed renderowaniem.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtr HTML dla całego widżetu podczas renderowania.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (PRZESTARZAŁE) Zwraca HTML do dodania na górze obszaru odpowiedzi.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (PRZESTARZAŁE) Zwraca HTML do dodania na górze widżetu.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (PRZESTARZAŁE) Zwraca HTML do dodania na górze elementu komentarza.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (PRZESTARZAŁE) Zwraca HTML do dodania na dole elementu komentarza.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (PRZESTARZAŁE) Zwraca HTML do dodania na dole elementu menu dla każdego komentarza.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Element root.
 * @param {Object.<string, Function>} clickListeners - Obsługiwacze zdarzeń kliknięć, według nazwy klasy, które można modyfikować przez referencję.
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