L'oggetto extension è composto dalla seguente definizione:

<!-- se vuoi aggiornarla, ricorda di aggiornare comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc dell\'oggetto Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * L'oggetto extension di FastCommentsUI. Utilizzato per il lazy-loading di alcuni componenti. Ad esempio, il sistema di recensioni non è
 * usato da tutti i clienti, quindi carichiamo quella extension solo quando la vogliamo.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Il nodo DOM radice del widget.
 * @property {string} [css]
 * @property {Object} config - L'oggetto config di FastComments.
 * @property {Object} commentsById - Un riferimento a un oggetto con tutti i commenti per id, che viene tenuto aggiornato.
 * @property {Object} translations - Un riferimento a tutte le traduzioni.
 * @property {Function} reRenderComment - Un riferimento a una funzione che può essere invocata per ri-renderizzare un commento.
 * @property {Function} removeCommentAndReRender - Un riferimento a una funzione che può essere invocata per rimuovere un commento dalla memoria e ri-renderizzare la parte appropriata del DOM.
 * @property {Function} newBroadcastId - Un riferimento a una funzione che può essere invocata per creare un nuovo broadcast id e aggiungerlo alla lista locale di broadcast id da ignorare.
 * @property {Function} [withSessionId] - Da chiamare con un URL di richiesta e una callback. Invoca la callback con l'URL più l'id di sessione del visualizzatore aggiunto quando i cookie di terze parti sono bloccati (lo stesso percorso usato dalle richieste di commento e voto), oppure l'URL invariato in caso contrario. Non usato per SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Chiamata con il commento che sta per essere pubblicato. Restituire false per annullare l'invio (ad esempio quando un sondaggio allegato è incompleto).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtra l'HTML dell'area dei commenti.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtra l'HTML dell'intero widget al render.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtra l'HTML di ogni commento prima del render.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtra l'HTML del menu di ogni commento prima del render.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtra l'HTML dell'intero widget al render.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Restituisce HTML da aggiungere in cima all'area di risposta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Restituisce HTML da aggiungere in cima al widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Restituisce HTML da aggiungere in cima all'elemento del commento.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Restituisce HTML da aggiungere in fondo all'elemento del commento.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Restituisce HTML da aggiungere dopo il testo del commento, dentro l'elemento del contenuto del commento (usato dai sondaggi).
 * @property {Function} [replyAreaInputBottom] - Restituisce HTML da aggiungere dentro il frame di input dei commenti, sotto il campo di testo (usato dai sondaggi per l'editor del sondaggio in-place). Riceve l'id del commento padre, oppure null per la casella di risposta principale.
 * @property {Function} [onPollUpdate] - Chiamata con l'evento live quando i conteggi dei voti di un sondaggio nella pagina cambiano.
 * @property {Function} isSiteAdmin - Restituisce se il visualizzatore è un admin o un moderatore del tenant. Nota dopo il primo fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Restituisce HTML da aggiungere in fondo all'elemento del menu per ogni commento.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - L'elemento radice.
 * @param {Object.<string, Function>} clickListeners - Gli event handler per i click, per nome classe, che possono essere modificati per riferimento.
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