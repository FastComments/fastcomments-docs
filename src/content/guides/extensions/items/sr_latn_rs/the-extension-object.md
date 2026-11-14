Objekat proširenja (extension) sastoji se od sledeće definicije:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc dokumentacija objekta proširenja'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Objekat proširenja FastCommentsUI. Koristi se za lenje učitavanje određenih komponenti. Na primer, sistem recenzija ne
 * koriste svi korisnici, pa taj extension učitavamo samo kada nam je potreban.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Koreni DOM čvor vidžeta.
 * @property {string} [css]
 * @property {Object} config - FastComments config objekat.
 * @property {Object} commentsById - Referenca na objekat sa svim komentarima po id-ju, koji se stalno ažurira.
 * @property {Object} translations - Referenca na sve prevode.
 * @property {Function} reRenderComment - Referenca na funkciju koja se može pozvati da ponovo renderuje komentar.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koja se može pozvati da ukloni komentar iz memorije i ponovo renderuje odgovarajući deo DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkciju koja kreira novi broadcast id i dodaje ga u lokalnu listu broadcast id-jeva koje treba ignorisati.
 * @property {Function} [withSessionId] - Poziva se sa URL-om zahteva i callback funkcijom. Poziva callback sa URL-om plus id-jem sesije gledaoca kada su third-party kolačići blokirani (isti put koji koriste zahtevi za komentar i glasanje), ili sa nepromenjenim URL-om u suprotnom. Ne koristi se za SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Poziva se sa komentarom koji će biti objavljen. Vratite false da otkažete slanje (na primer kada je priložena anketa nepotpuna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za oblast komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za ceo vidžet pri renderovanju.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za svaki komentar pre renderovanja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za meni svakog komentara pre renderovanja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za ceo vidžet pri renderovanju.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ZASTARELO) Vraća HTML koji se dodaje na vrh oblasti za odgovor.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ZASTARELO) Vraća HTML koji se dodaje na vrh vidžeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ZASTARELO) Vraća HTML koji se dodaje na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ZASTARELO) Vraća HTML koji se dodaje na dno elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vraća HTML koji se dodaje posle teksta komentara, unutar elementa sadržaja komentara (koriste se ankete).
 * @property {Function} [replyAreaInputBottom] - Vraća HTML koji se dodaje unutar okvira za unos komentara, ispod polja za tekst (koriste se ankete za editor ankete na licu mesta). Prima id roditeljskog komentara, ili null za glavni okvir za odgovor.
 * @property {Function} [onPollUpdate] - Poziva se sa live događajem kada se promene brojači glasova ankete na stranici.
 * @property {Function} isSiteAdmin - Vraća da li je gledalac admin ili moderator tenanta. Postaje poznato posle prvog preuzimanja podataka.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ZASTARELO) Vraća HTML koji se dodaje na dno elementa menija za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Koreni element.
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