The extension object consists of the following definition:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Objekat Ekstenzije JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI ekstenzioni objekat. Koristi se za lenjo učitavanje određenih komponenti. Na primer, sistem za recenzije nije
 * korišćen od svih kupaca, pa učitavamo tu ekstenziju samo kada nam je potrebna.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - DOM čvor korena vidžeta.
 * @property {string} [css]
 * @property {Object} config - FastComments konfiguracioni objekat.
 * @property {Object} commentsById - Referenca na objekat sa svim komentarima po ID‑u, koji se ažurira u realnom vremenu.
 * @property {Object} translations - Referenca na sve prevode.
 * @property {Function} reRenderComment - Referenca na funkciju koja može da se pozove za ponovno renderovanje komentara.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koja može da se pozove za uklanjanje komentara iz memorije i ponovno renderovanje odgovarajućeg dela DOM‑a.
 * @property {Function} newBroadcastId - Referenca na funkciju koja može da se pozove za kreiranje novog broadcast ID‑a i dodavanje u lokalnu listu broadcast ID‑ova koje treba ignorisati.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Poziva se sa komentarom koji će biti objavljen. Vratite false da otkažete slanje (na primer kada je priložena anketa nekompletna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrirajte HTML za oblast komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrirajte HTML za ceo vidžet prilikom renderovanja.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrirajte HTML za svaki komentar pre renderovanja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrirajte HTML za svaki meni komentara pre renderovanja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrirajte HTML za ceo vidžet prilikom renderovanja.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Vraća HTML koji se dodaje na vrh oblasti za odgovor.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Vraća HTML koji se dodaje na vrh vidžeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Vraća HTML koji se dodaje na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Vraća HTML koji se dodaje na dno elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vraća HTML koji se dodaje posle teksta komentara, unutar elementa sadržaja komentara (koristi se za ankete).
 * @property {Function} [replyAreaInputBottom] - Vraća HTML koji se dodaje unutar okvira za unos komentara, ispod polja za tekst (koristi se za ankete u editoru ankete na mestu). Prima ID roditeljskog komentara, ili null za korenski okvir za odgovor.
 * @property {Function} [onPollUpdate] - Poziva se sa live događajem kada se promeni broj glasova ankete na stranici.
 * @property {Function} isSiteAdmin - Vraća da li je posmatrač administrator ili moderator tenant‑a. Poznato nakon prvog fetch‑a.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Vraća HTML koji se dodaje na dno menija za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korenski element.
 * @param {Object.<string, Function>} clickListeners - Handleri za klikove, po nazivu klase, koji se mogu izmeniti po referenci.
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