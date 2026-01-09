Objekt proširenja sastoji se od sljedeće definicije:

<!-- ako želite ažurirati ovo, zapamtite ažurirati comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc objekta proširenja'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Korijenski DOM čvor widgeta.
 * @property {string} [css]
 * @property {Object} config - FastComments konfiguracijski objekt.
 * @property {Object} commentsById - Referenca na objekt sa svim komentarima po ID-u, koji se održava ažurnim.
 * @property {Object} translations - Referenca na sve prijevode.
 * @property {Function} reRenderComment - Referenca na funkciju koja se može pozvati za ponovno renderiranje komentara.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koja se može pozvati za uklanjanje komentara iz memorije i ponovno renderiranje odgovarajućeg dijela DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkciju koja se može pozvati za stvaranje novog broadcast ID-a i dodavanje u lokalnu listu broadcast ID-eva koje treba ignorirati.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za područje komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za cijeli widget pri renderiranju.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za svaki komentar prije renderiranja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za svaki izbornik komentara prije renderiranja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za cijeli widget pri renderiranju.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ZASTARJELO) Vraća HTML za dodavanje na vrh područja odgovora.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ZASTARJELO) Vraća HTML za dodavanje na vrh widgeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ZASTARJELO) Vraća HTML za dodavanje na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ZASTARJELO) Vraća HTML za dodavanje na dno elementa komentara.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ZASTARJELO) Vraća HTML za dodavanje na dno elementa izbornika za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korijenski element.
 * @param {Object.<string, Function>} clickListeners - Handleri događaja za klikove, po nazivu klase, koje je moguće mijenjati preko reference.
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