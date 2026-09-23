Objekt proširenja sastoji se od sljedeće definicije:

<!-- ako želite ažurirati ovo, zapamtite ažurirati comment-ui-core -->
[inline-code-attrs-start title = 'Objekt proširenja JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI objekt proširenja. Koristi se za lijeno učitavanje određenih komponenti. Na primjer, sustav recenzija nije korišten od svih kupaca, pa učitavamo to proširenje samo kada ga trebamo.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - DOM čvor korijena widgeta.
 * @property {string} [css]
 * @property {Object} config - FastComments konfiguracijski objekt.
 * @property {Object} commentsById - Referenca na objekt koji sadrži sve komentare po ID-u, koji se održava ažurnim.
 * @property {Object} translations - Referenca na sve prijevode.
 * @property {Function} reRenderComment - Referenca na funkciju koja se može pozvati za ponovno renderiranje komentara.
 * @property {Function} removeCommentAndReRender - Referenca na funkciju koja se može pozvati za uklanjanje komentara iz memorije i ponovno renderiranje odgovarajućeg dijela DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkciju koja se može pozvati za stvaranje novog broadcast ID-a i dodavanje u lokalni popis broadcast ID-ova koje treba ignorirati.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Poziva se s komentarom koji će biti objavljen. Vraća false za otkazivanje slanja (na primjer kada je priloženi upitnik nepotpun).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za područje komentara.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za cijeli widget prilikom renderiranja.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za svaki komentar prije renderiranja.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za svaki izbornik komentara prije renderiranja.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za cijeli widget prilikom renderiranja.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Vraća HTML koji se dodaje na vrh područja odgovora.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Vraća HTML koji se dodaje na vrh widgeta.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Vraća HTML koji se dodaje na vrh elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Vraća HTML koji se dodaje na dno elementa komentara.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vraća HTML koji se dodaje nakon teksta komentara, unutar elementa sadržaja komentara (koristi se u anketama).
 * @property {Function} [replyAreaInputBottom] - Vraća HTML koji se dodaje unutar okvira za unos komentara, ispod tekstualnog polja (koristi se u anketama za uređivač ankete na mjestu). Prima ID roditeljskog komentara ili null za korijenski okvir odgovora.
 * @property {Function} [onPollUpdate] - Poziva se s događajem uživo kada se promijeni broj glasova ankete na stranici.
 * @property {Function} isSiteAdmin - Vraća je li gledatelj administrator ili moderator najamnika. Poznato nakon prvog dohvaćanja.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Vraća HTML koji se dodaje na dno izbornika za svaki komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korijenski element.
 * @param {Object.<string, Function>} clickListeners - Obrađivači događaja za klikove, po nazivu klase, koji se mogu modificirati po referenci.
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