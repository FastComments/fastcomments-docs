Objekt proširenja sastoji se od sljedeće definicije:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc dokumentacija objekta Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Objekt proširenja FastCommentsUI. Koristi se za lazy-loading određenih komponenti. Na primjer, sustav recenzija ne
 * koriste svi kupci, pa taj extension učitavamo samo kada nam treba.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Korijenski DOM čvor widgeta.
 * @property {string} [css]
 * @property {Object} config - FastComments config objekt.
 * @property {Object} commentsById - Referenca na objekt sa svim komentarima po id-u, koji se stalno ažurira.
 * @property {Object} translations - Referenca na sve prijevode.
 * @property {Function} reRenderComment - Referenca na funkciju koja se može pozvati za ponovno renderiranje komentara.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koja se može pozvati za uklanjanje komentara iz memorije i ponovno renderiranje odgovarajućeg dijela DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkciju koja se može pozvati za stvaranje novog broadcast id-a i dodavanje u lokalni popis broadcast id-ova koje treba ignorirati.
 * @property {Function} [withSessionId] - Poziva se s URL-om zahtjeva i callback funkcijom. Poziva callback s URL-om plus viewerovim session id-om dodanim kada su third-party kolačići blokirani (isti put koji koriste zahtjevi za komentare i glasove), ili s nepromijenjenim URL-om inače. Kada session id nije na taj način dostupan, dodaje zadnji anonimni session id dan funkciji rememberAnonSessionId, ako postoji. Ne koristi se za SSO.
 * @property {Function} [rememberAnonSessionId] - Poziva se s anonSessionId koji je odgovor vratio. Server ga vraća samo gostu čiji zahtjev nije imao session kolačić, i nikada za prijavljenu sesiju. Čuva se u localStorage (ili u memoriji kada pohrana nije dostupna) tako da withSessionId šalje id u kasnijim zahtjevima i gost zadržava jedan identitet.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Poziva se s komentarom koji će biti objavljen. Vratite false za otkazivanje slanja (na primjer kada je priložena anketa nepotpuna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za područje komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za cijeli widget prilikom renderiranja.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za svaki komentar prije renderiranja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za izbornik svakog komentara prije renderiranja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za cijeli widget prilikom renderiranja.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Vraća HTML koji se dodaje na vrh područja za odgovor.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Vraća HTML koji se dodaje na vrh widgeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Vraća HTML koji se dodaje na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Vraća HTML koji se dodaje na dno elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vraća HTML koji se dodaje nakon teksta komentara, unutar elementa sadržaja komentara (koriste se u anketama).
 * @property {Function} [replyAreaInputBottom] - Vraća HTML koji se dodaje unutar okvira za unos komentara, ispod polja za unos teksta (koriste se u anketama za uređivač ankete na mjestu). Prima id roditeljskog komentara ili null za glavni okvir za odgovor.
 * @property {Function} [onPollUpdate] - Poziva se s live eventom kada se promijene brojevi glasova ankete na stranici.
 * @property {Function} isSiteAdmin - Vraća je li gledatelj admin ili moderator tenant-a. Poznato nakon prvog dohvata.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Vraća HTML koji se dodaje na dno elementa izbornika za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korijenski element.
 * @param {Object.<string, Function>} clickListeners - Rukovatelji događajima za klikove, po nazivu klase, koje se mogu mijenjati po referenci.
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