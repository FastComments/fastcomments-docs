The extension object consists of the following definition:

<!-- ako želite ovo da ažurirate, zapamtite da ažurirate comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc objekta ekstenzije'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Korenski DOM čvor widgeta.
 * @property {string} [css]
 * @property {Object} config - FastComments konfiguracioni objekat.
 * @property {Object} commentsById - Referenca na objekat sa svim komentarima po id-ju, koja se održava ažurnom.
 * @property {Object} translations - Referenca na sve prevode.
 * @property {Function} reRenderComment - Referenca na funkciju koju je moguće pozvati da se ponovo renderuje komentar.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koju je moguće pozvati da ukloni komentar iz memorije i ponovo renderuje odgovarajući deo DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkciju koju je moguće pozvati da kreira novi broadcast id i doda ga u lokalnu listu broadcast id-eva koje treba ignorisati.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filter HTML-a za oblast komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filter HTML-a za ceo widget pri renderovanju.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filter HTML-a za svaki komentar pre renderovanja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filter HTML-a za svaki meni komentara pre renderovanja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filter HTML-a za ceo widget pri renderovanju.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ZASTARELO) Vraća HTML koji će biti dodat na vrh polja za odgovor.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ZASTARELO) Vraća HTML koji će biti dodat na vrh widgeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ZASTARELO) Vraća HTML koji će biti dodat na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ZASTARELO) Vraća HTML koji će biti dodat na dno elementa komentara.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ZASTARELO) Vraća HTML koji će biti dodat na dno elementa menija za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korenski element.
 * @param {Object.<string, Function>} clickListeners - Obradivači događaja za klikove, po nazivu klase, koje je moguće izmeniti putem reference.
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