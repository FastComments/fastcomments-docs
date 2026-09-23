[inline-code-attrs-start title = 'Razširjeni objekt JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Razširjeni objekt FastCommentsUI. Uporabljen za leno nalaganje določenih komponent. Na primer, sistem za ocenjevanje ni uporabljen pri vseh strankah, zato to razširitev naložimo le, ko jo potrebujemo.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Korenni DOM vozlišče gradnika.
 * @property {string} [css]
 * @property {Object} config - Konfiguracijski objekt FastComments.
 * @property {Object} commentsById - Referenca na objekt, ki vsebuje vse komentarje po ID-ju in je vzdrževana posodobljena.
 * @property {Object} translations - Referenca na vse prevode.
 * @property {Function} reRenderComment - Referenca na funkcijo, ki jo je mogoče poklicati za ponovno upodabljanje komentarja.
 * @property {Function} removeCommentAndReRender - Referenca na funkcijo, ki jo je mogoče poklicati za odstranitev komentarja iz pomnilnika in ponovno upodabljanje ustreznega dela DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkcijo, ki jo je mogoče poklicati za ustvarjanje novega ID-ja za oddajanje in dodajanje v lokalni seznam ID-jev oddajanja, ki jih je treba prezreti.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Poklicana s komentarjem, ki naj bo objavljen. Vrne false za preklic pošiljanja (na primer, ko je priloženo glasovanje nepopolno).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za območje komentarja.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za celoten gradnik ob upodabljanju.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za vsak komentar pred upodabljanjem.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za vsak meni komentarja pred upodabljanjem.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za celoten gradnik ob upodabljanju.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ZASTARJEL) Vrne HTML, ki se doda na vrh območja odgovora.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ZASTARJEL) Vrne HTML, ki se doda na vrh gradnika.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ZASTARJEL) Vrne HTML, ki se doda na vrh elementa komentarja.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ZASTARJEL) Vrne HTML, ki se doda na dno elementa komentarja.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vrne HTML, ki se doda po besedilu komentarja, znotraj elementa vsebine komentarja (uporablja se pri anketah).
 * @property {Function} [replyAreaInputBottom] - Vrne HTML, ki se doda znotraj okvira za vnos komentarja, pod besedilnim vnosom (uporablja se pri anketah za vgrajeni urejevalnik anket). Prejme ID nadrejenega komentarja ali null za korensko polje odgovora.
 * @property {Function} [onPollUpdate] - Poklicano z dogodkom v živo, ko se število glasov v anketi na strani spremeni.
 * @property {Function} isSiteAdmin - Vrne, ali je gledalec skrbnik ali moderator najemnika. Znano po prvem pridobivanju.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ZASTARJEL) Vrne HTML, ki se doda na dno elementa menija za vsak komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korenni element.
 * @param {Object.<string, Function>} clickListeners - Obdelovalci dogodkov za klike, po imenu razreda, ki jih je mogoče spremeniti po referenci.
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