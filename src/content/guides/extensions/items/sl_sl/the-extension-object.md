Objekt extension je sestavljen iz naslednje definicije:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc dokumentacija objekta Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Objekt extension za FastCommentsUI. Uporablja se za lenobno nalaganje določenih komponent. Na primer, sistem ocen
 * ne uporabljajo vsi stranke, zato ta extension naložimo šele, ko ga potrebujemo.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Korensko DOM vozlišče pripomočka (widget).
 * @property {string} [css]
 * @property {Object} config - Konfiguracijski objekt FastComments.
 * @property {Object} commentsById - Referenca na objekt z vsemi komentarji po id, ki se stalno posodablja.
 * @property {Object} translations - Referenca na vse prevode.
 * @property {Function} reRenderComment - Referenca na funkcijo, ki jo je mogoče poklicati za ponovno izris komentarja.
 * @property {Function} removeCommentAndReRender - Referenca na funkcijo, ki jo je mogoče poklicati za odstranitev komentarja iz pomnilnika in ponovni izris ustreznega dela DOM.
 * @property {Function} newBroadcastId - Referenca na funkcijo, ki jo je mogoče poklicati za ustvarjanje novega broadcast id in njegovo dodajanje v lokalni seznam broadcast id, ki jih želimo ignorirati.
 * @property {Function} [withSessionId] - Pokličite z URL zahteve in povratnim klicem. Povratni klic se izvede z URL-jem, ki mu je dodan id seje gledalca, ko so blokirani piškotki tretjih oseb (ista pot, ki jo uporabljajo zahteve za komentarje in glasovanje), sicer pa se URL vrne nespremenjen. Ni uporabljeno za SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Pokliče se s komentarjem, ki ga je treba objaviti. Vrnite false za preklic oddaje (na primer, kadar je priložena anketa nepopolna).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrira HTML za območje komentarjev.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrira HTML za celoten pripomoček ob izrisu.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrira HTML za vsak komentar pred izrisom.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrira HTML za meni vsakega komentarja pred izrisom.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrira HTML za celoten pripomoček ob izrisu.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ZASTARELO) Vrne HTML, ki se doda na vrh območja za odgovore.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ZASTARELO) Vrne HTML, ki se doda na vrh pripomočka.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ZASTARELO) Vrne HTML, ki se doda na vrh elementa komentarja.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ZASTARELO) Vrne HTML, ki se doda na dno elementa komentarja.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Vrne HTML, ki se doda za besedilom komentarja, znotraj elementa vsebine komentarja (uporabljajo ankete).
 * @property {Function} [replyAreaInputBottom] - Vrne HTML, ki se doda znotraj okvira vnosa komentarja, pod besedilnim vnosom (uporabljajo ankete za urejevalnik ankete na mestu). Prejme id nadrejenega komentarja ali null za korensko polje za odgovore.
 * @property {Function} [onPollUpdate] - Pokliče se z dogodkom v živo, ko se spremenijo števila glasov ankete na strani.
 * @property {Function} isSiteAdmin - Vrne, ali je gledalec skrbnik ali moderator najemnika. Znano po prvem pridobivanju podatkov.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ZASTARELO) Vrne HTML, ki se doda na dno elementa menija za vsak komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Korenski element.
 * @param {Object.<string, Function>} clickListeners - Upravljalniki dogodkov za klike, po imenu razreda, ki jih je mogoče spreminjati po referenci.
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