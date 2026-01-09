Objekt razširitve vsebuje naslednjo definicijo:

<!-- če želite to posodobiti, ne pozabite posodobiti comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc razširitvenega objekta'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Uporablja se za odloženo nalaganje določenih komponent. Na primer, sistem ocen ni
 * v uporabi pri vseh strankah, zato to razširitev naložimo le takrat, ko jo potrebujemo.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Koreninski DOM element gradnika.
 * @property {string} [css]
 * @property {Object} config - Konfiguracijski objekt FastComments.
 * @property {Object} commentsById - Referenca na objekt z vsemi komentarji po id-ju, ki se ažurno posodablja.
 * @property {Object} translations - Referenca na vse prevode.
 * @property {Function} reRenderComment - Referenca na funkcijo, ki jo lahko pokličete za ponovni izris komentarja.
 * @property {Function} removeCommentAndReRender - Referenca na funkcijo, ki jo lahko pokličete za odstranitev komentarja iz pomnilnika in ponovno upodobitev ustreznega dela DOM-a.
 * @property {Function} newBroadcastId - Referenca na funkcijo, ki jo lahko pokličete za ustvarjanje novega broadcast id-ja in njegovo dodajanje v lokalni seznam broadcast id-jev, ki jih je treba prezreti.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filter HTML-ja za območje komentarjev.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filter HTML-ja za celoten gradnik ob upodabljanju.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filter HTML-ja za vsak komentar pred upodabljanjem.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filter HTML-ja za meni vsakega komentarja pred upodabljanjem.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filter HTML-ja za celoten gradnik ob upodabljanju.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ZASTARELO) Vrne HTML za dodajanje na vrh območja odgovorov.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ZASTARELO) Vrne HTML za dodajanje na vrh gradnika.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ZASTARELO) Vrne HTML za dodajanje na vrh elementa komentarja.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ZASTARELO) Vrne HTML za dodajanje na dno elementa komentarja.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ZASTARELO) Vrne HTML za dodajanje na dno elementa menija za vsak komentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Koreninski element.
 * @param {Object.<string, Function>} clickListeners - Event handlerji za klike, po imenu razreda, ki jih je mogoče spremeniti po referenci.
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