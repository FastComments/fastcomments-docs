L'oggetto extension è composto dalla seguente definizione:

<!-- se vuoi aggiornare questo, ricorda di aggiornare comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc oggetto Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Il nodo root DOM del widget.
 * @property {string} [css]
 * @property {Object} config - L'oggetto di configurazione FastComments.
 * @property {Object} commentsById - Riferimento a un oggetto con tutti i commenti per id, mantenuto aggiornato.
 * @property {Object} translations - Riferimento a tutte le traduzioni.
 * @property {Function} reRenderComment - Riferimento a una funzione che può essere invocata per ri-renderizzare un commento.
 * @property {Function} removeCommentAndReRender - Riferimento a una funzione che può essere invocata per rimuovere un commento dalla memoria e ri-renderizzare la parte appropriata del DOM.
 * @property {Function} newBroadcastId - Riferimento a una funzione che può essere invocata per creare un nuovo broadcast id e aggiungerlo alla lista locale di broadcast id da ignorare.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtra l'HTML per l'area dei commenti.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtra l'HTML per l'intero widget al render.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtra l'HTML per ogni commento prima del render.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtra l'HTML per ogni menu del commento prima del render.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtra l'HTML per l'intero widget al render.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Restituisce HTML da aggiungere in cima all'area di risposta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Restituisce HTML da aggiungere in cima al widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Restituisce HTML da aggiungere in cima all'elemento commento.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Restituisce HTML da aggiungere in fondo all'elemento commento.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Restituisce HTML da aggiungere in fondo all'elemento menu per ogni commento.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - L'elemento root.
 * @param {Object.<string, Function>} clickListeners - I gestori degli eventi per i click, per nome di classe, che possono essere modificati per riferimento.
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