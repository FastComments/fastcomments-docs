Obiekt rozszerzenia składa się z następującej definicji:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Dokumentacja JSDoc obiektu Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Obiekt rozszerzenia FastCommentsUI. Używany do leniwego ładowania określonych komponentów. Na przykład system recenzji nie jest
 * używany przez wszystkich klientów, więc ładujemy to rozszerzenie tylko wtedy, gdy jest potrzebne.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Węzeł DOM root widżetu.
 * @property {string} [css]
 * @property {Object} config - Obiekt konfiguracyjny FastComments.
 * @property {Object} commentsById - Referencja do obiektu ze wszystkimi komentarzami według id, który jest na bieżąco aktualizowany.
 * @property {Object} translations - Referencja do wszystkich tłumaczeń.
 * @property {Function} reRenderComment - Referencja do funkcji, którą można wywołać, aby ponownie wyrenderować komentarz.
 * @property {Function} removeCommentAndReRender - Referencja do funkcji, którą można wywołać, aby usunąć komentarz z pamięci i ponownie wyrenderować odpowiednią część DOM.
 * @property {Function} newBroadcastId - Referencja do funkcji, którą można wywołać, aby utworzyć nowy broadcast id i dodać go do lokalnej listy broadcast id do ignorowania.
 * @property {Function} [withSessionId] - Wywołaj z adresem URL żądania i callbackiem. Wywołuje callback z URL plus dołączonym id sesji przeglądającego, gdy ciasteczka stron trzecich są blokowane (ta sama ścieżka, której używają żądania komentarzy i głosów), lub niezmienionym URL w przeciwnym razie. Gdy w ten sposób nie jest dostępne żadne id sesji, dołącza ostatnie anonimowe id sesji przekazane do rememberAnonSessionId, jeśli istnieje. Nieużywane dla SSO.
 * @property {Function} [rememberAnonSessionId] - Wywołaj z anonSessionId zwróconym przez odpowiedź. Serwer zwraca je tylko gościowi, którego żądanie nie miało ciasteczka sesji, i nigdy dla zalogowanej sesji. Jest przechowywane w localStorage (lub w pamięci, gdy magazyn jest niedostępny), dzięki czemu withSessionId wysyła je w kolejnych żądaniach, a gość zachowuje jedną tożsamość.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Wywoływane z komentarzem, który ma zostać opublikowany. Zwróć false, aby anulować wysłanie (na przykład gdy dołączona ankieta jest niekompletna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtruje HTML dla obszaru komentarzy.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtruje HTML dla całego widżetu podczas renderowania.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtruje HTML dla każdego komentarza przed renderowaniem.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtruje HTML dla menu każdego komentarza przed renderowaniem.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtruje HTML dla całego widżetu podczas renderowania.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Zwraca HTML do dodania na górze obszaru odpowiedzi.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Zwraca HTML do dodania na górze widżetu.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Zwraca HTML do dodania na górze elementu komentarza.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Zwraca HTML do dodania na dole elementu komentarza.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Zwraca HTML do dodania po tekście komentarza, wewnątrz elementu treści komentarza (używane przez ankiety).
 * @property {Function} [replyAreaInputBottom] - Zwraca HTML do dodania wewnątrz ramki pola komentarza, pod polem tekstowym (używane przez ankiety dla edytora ankiety w miejscu). Otrzymuje id komentarza nadrzędnego lub null dla głównego pola odpowiedzi.
 * @property {Function} [onPollUpdate] - Wywoływane ze zdarzeniem live, gdy zmieniają się liczby głosów ankiety na stronie.
 * @property {Function} isSiteAdmin - Zwraca informację, czy przeglądający jest administratorem lub moderatorem tenanta. Znane po pierwszym pobraniu.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Zwraca HTML do dodania na dole elementu menu dla każdego komentarza.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Element główny.
 * @param {Object.<string, Function>} clickListeners - Handlery zdarzeń kliknięć, według nazwy klasy, które mogą być modyfikowane przez referencję.
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