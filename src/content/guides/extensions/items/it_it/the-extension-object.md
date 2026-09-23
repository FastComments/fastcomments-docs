L'oggetto di estensione consiste nella seguente definizione:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Oggetto Estensione JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * L'oggetto di estensione FastCommentsUI. Utilizzato per il caricamento lazy di alcuni componenti. Per esempio, il sistema di recensioni non
 * è usato da tutti i clienti, quindi carichiamo quell'estensione solo quando ne abbiamo bisogno.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Il nodo DOM radice del widget.
 * @property {string} [css]
 * @property {Object} config - L'oggetto di configurazione FastComments.
 * @property {Object} commentsById - Un riferimento a un oggetto con tutti i commenti per id, mantenuto aggiornato.
 * @property {Object} translations - Un riferimento a tutte le traduzioni.
 * @property {Function} reRenderComment - Un riferimento a una funzione che può essere invocata per ridisegnare un commento.
 * @property {Function} removeCommentAndReRender - Un riferimento a una funzione che può essere invocata per rimuovere un commento dalla memoria e ridisegnare la parte appropriata del DOM.
 * @property {Function} newBroadcastId - Un riferimento a una funzione che può essere invocata per creare un nuovo broadcast id e aggiungerlo alla lista locale di broadcast id da ignorare.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Chiamata con il commento che sta per essere pubblicato. Restituisce false per annullare l'invio (ad esempio quando un sondaggio allegato è incompleto).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtra l'HTML per l'area del commento.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtra l'HTML per l'intero widget al rendering.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtra l'HTML per ogni commento prima del rendering.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtra l'HTML per ogni menu del commento prima del rendering.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtra l'HTML per l'intero widget al rendering.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Restituisce HTML da aggiungere in cima all'area di risposta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Restituisce HTML da aggiungere in cima al widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Restituisce HTML da aggiungere in cima all'elemento del commento.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Restituisce HTML da aggiungere in fondo all'elemento del commento.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Restituisce HTML da aggiungere dopo il testo del commento, all'interno dell'elemento del contenuto del commento (usato dai sondaggi).
 * @property {Function} [replyAreaInputBottom] - Restituisce HTML da aggiungere all'interno del frame di input del commento, sotto l'input di testo (usato dai sondaggi per l'editor di sondaggio in loco). Riceve l'id del commento genitore, o null per la casella di risposta radice.
 * @property {Function} [onPollUpdate] - Chiamata con l'evento live quando i conteggi dei voti di un sondaggio sulla pagina cambiano.
 * @property {Function} isSiteAdmin - Restituisce se lo spettatore è un amministratore o moderatore del tenant. Conosciuto dopo il primo fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Restituisce HTML da aggiungere in fondo all'elemento del menu per ogni commento.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - L'elemento radice.
 * @param {Object.<string, Function>} clickListeners - I gestori di eventi per i click, per nome di classe, che possono essere modificati per riferimento.
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