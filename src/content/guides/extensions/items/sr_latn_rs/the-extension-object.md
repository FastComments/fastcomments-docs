Objekat proširenja (extension) sastoji se iz sledeće definicije:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc dokumentacija objekta proširenja'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Objekat proširenja FastCommentsUI. Koristi se za lenje učitavanje određenih komponenti. Na primer, sistem recenzija ne
 * koriste svi korisnici, pa taj extension učitavamo samo kada nam treba.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Root DOM čvor vidžeta.
 * @property {string} [css]
 * @property {Object} config - FastComments config objekat.
 * @property {Object} commentsById - Referenca na objekat sa svim komentarima po id-ju, koji se stalno ažurira.
 * @property {Object} translations - Referenca na sve prevode.
 * @property {Function} reRenderComment - Referenca na funkciju koja se može pozvati da ponovo renderuje komentar.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koja se može pozvati da ukloni komentar iz memorije i ponovo renderuje odgovarajući deo DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkciju koja se može pozvati da kreira novi broadcast id i doda ga u lokalnu listu broadcast id-jeva koje treba ignorisati.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Poziva se sa komentarom koji će biti objavljen. Vratite false da otkažete slanje (na primer kada je priložena anketa nepotpuna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za oblast komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za ceo vidžet pri renderovanju.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za svaki komentar pre renderovanja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za meni svakog komentara pre renderovanja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za ceo vidžet pri renderovanju.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Vraća HTML koji se dodaje na vrh oblasti za odgovore.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Vraća HTML koji se dodaje na vrh vidžeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Vraća HTML koji se dodaje na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Vraća HTML koji se dodaje na dno elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vraća HTML koji se dodaje posle teksta komentara, unutar elementa sadržaja komentara (koriste ankete).
 * @property {Function} [replyAreaInputBottom] - Vraća HTML koji se dodaje unutar okvira za unos komentara, ispod polja za tekst (koriste ankete za uređivač ankete na licu mesta). Prima id roditeljskog komentara, ili null za glavni okvir za odgovore.
 * @property {Function} [onPollUpdate] - Poziva se sa live eventom kada se promene brojaci glasova ankete na stranici.
 * @property {Function} isSiteAdmin - Vraća da li je posmatrač admin ili moderator tenanta. Poznato posle prvog fetch-a.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Vraća HTML koji se dodaje na dno elementa menija za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Root element.
 * @param {Object.<string, Function>} clickListeners - Handleri događaja za klikove, po imenu klase, koje je moguće menjati po referenci.
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