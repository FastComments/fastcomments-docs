Obiekt rozszerzenia składa się z następującej definicji:

<!-- jeśli chcesz zaktualizować to, pamiętaj, aby zaktualizować comment-ui-core -->
[inline-code-attrs-start title = 'Obiekt Rozszerzenia JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Obiekt rozszerzenia FastCommentsUI. Używany do leniwego ładowania niektórych komponentów. Na przykład system recenzji nie jest używany przez wszystkich klientów, więc ładujemy to rozszerzenie tylko wtedy, gdy jest potrzebne.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Główny węzeł DOM widżetu.
 * @property {string} [css]
 * @property {Object} config - Obiekt konfiguracyjny FastComments.
 * @property {Object} commentsById - Odniesienie do obiektu zawierającego wszystkie komentarze według identyfikatora, który jest na bieżąco aktualizowany.
 * @property {Object} translations - Odniesienie do wszystkich tłumaczeń.
 * @property {Function} reRenderComment - Odniesienie do funkcji, którą można wywołać, aby ponownie wyrenderować komentarz.
 * @property {Function} removeCommentAndReRender - Odniesienie do funkcji, którą można wywołać, aby usunąć komentarz z pamięci i ponownie wyrenderować odpowiednią część DOM.
 * @property {Function} newBroadcastId - Odniesienie do funkcji, którą można wywołać, aby utworzyć nowy identyfikator transmisji i dodać go do lokalnej listy identyfikatorów transmisji do ignorowania.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Wywoływany z komentarzem, który ma zostać opublikowany. Zwróć false, aby anulować wysyłkę (na przykład gdy dołączona ankieta jest niekompletna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtruje HTML dla obszaru komentarza.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtruje HTML dla całego widżetu podczas renderowania.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtruje HTML dla każdego komentarza przed renderowaniem.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtruje HTML dla każdego menu komentarza przed renderowaniem.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtruje HTML dla całego widżetu podczas renderowania.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Zwraca HTML do dodania na górze obszaru odpowiedzi.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Zwraca HTML do dodania na górze widżetu.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Zwraca HTML do dodania na górze elementu komentarza.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Zwraca HTML do dodania na dole elementu komentarza.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Zwraca HTML do dodania po tekście komentarza, wewnątrz elementu zawartości komentarza (używane w ankietach).
 * @property {Function} [replyAreaInputBottom] - Zwraca HTML do dodania wewnątrz ramki wprowadzania komentarza, pod polem tekstowym (używane w ankietach dla edytora ankiet w miejscu). Otrzymuje identyfikator komentarza nadrzędnego lub null dla głównego pola odpowiedzi.
 * @property {Function} [onPollUpdate] - Wywoływany przy zdarzeniu na żywo, gdy liczby głosów w ankiecie na stronie się zmieniają.
 * @property {Function} isSiteAdmin - Zwraca, czy oglądający jest administratorem lub moderatorem najemcy. Znane po pierwszym pobraniu.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Zwraca HTML do dodania na dole elementu menu dla każdego komentarza.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Główny element.
 * @param {Object.<string, Function>} clickListeners - Obsługa zdarzeń kliknięć, według nazwy klasy, które mogą być modyfikowane przez referencję.
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