Udvidelsesobjektet består af følgende definition:

<!-- hvis du vil opdatere dette, husk at opdatere comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc for Extension-objektet'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Widget-rodens DOM-node.
 * @property {string} [css]
 * @property {Object} config - FastComments konfigurationsobjektet.
 * @property {Object} commentsById - En reference til et objekt med alle kommentarer efter id, som holdes opdateret.
 * @property {Object} translations - En reference til alle oversættelser.
 * @property {Function} reRenderComment - En reference til en funktion, der kan kaldes for at gengive en kommentar igen.
 * @property {Function} removeCommentAndReRender - En reference til en funktion, der kan kaldes for at fjerne en kommentar fra hukommelsen og gengive den relevante del af DOM'en.
 * @property {Function} newBroadcastId - En reference til en funktion, der kan kaldes for at oprette et nyt broadcast-id og tilføje det til den lokale liste over broadcast-id'er, der skal ignoreres.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrer HTML for kommentarområdet.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrer HTML for hele widget'en ved gengivelse.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrer HTML for hver kommentar før gengivelse.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrer HTML for hver kommentarmenu før gengivelse.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrer HTML for hele widget'en ved gengivelse.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Returner HTML, der skal tilføjes øverst i svarområdet.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Returner HTML, der skal tilføjes øverst i widget'en.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Returner HTML, der skal tilføjes øverst i kommentarelementet.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Returner HTML, der skal tilføjes i bunden af kommentarelementet.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Returner HTML, der skal tilføjes i bunden af menuelementet for hver kommentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Rod-elementet.
 * @param {Object.<string, Function>} clickListeners - Eventhåndtererne for klik, efter klassenavn, som kan ændres ved reference.
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