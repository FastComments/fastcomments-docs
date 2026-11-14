Udvidelsesobjektet består af følgende definition:

<!-- hvis du vil opdatere denne, så husk at opdatere comment-ui-core -->
[inline-code-attrs-start title = 'Extension Object JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI-udvidelsesobjektet. Bruges til lazy-loading af visse komponenter. For eksempel bruger ikke alle
 * kunder anmeldessystemet, så vi indlæser kun den udvidelse, når vi har brug for den.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Widgettens rod-DOM-node.
 * @property {string} [css]
 * @property {Object} config - FastComments config-objektet.
 * @property {Object} commentsById - En reference til et objekt med alle kommentarer efter id, som holdes opdateret.
 * @property {Object} translations - En reference til alle oversættelser.
 * @property {Function} reRenderComment - En reference til en funktion, der kan kaldes for at re-rendere en kommentar.
 * @property {Function} removeCommentAndReRender - En reference til en funktion, der kan kaldes for at fjerne en kommentar fra hukommelsen og re-rendere den relevante del af DOM'en.
 * @property {Function} newBroadcastId - En reference til en funktion, der kan kaldes for at oprette et nyt broadcast-id og tilføje det til den lokale liste over broadcast-id'er, der skal ignoreres.
 * @property {Function} [withSessionId] - Kald med en request-URL og et callback. Kalder callbacket med URL'en plus seerens session-id tilføjet, når tredjepartscookies er blokeret (samme sti som kommentar- og vote-requests bruger), ellers uændret URL. Bruges ikke til SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Kaldes med kommentaren, der er ved at blive sendt. Returner false for at annullere afsendelsen (f.eks. når en vedhæftet afstemning er ufuldstændig).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrerer HTML for kommentarområdet.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrerer HTML for hele widgetten ved render.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrerer HTML for hver kommentar før render.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrerer HTML for hver kommentarmenu før render.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrerer HTML for hele widgetten ved render.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Returner HTML, der skal tilføjes i toppen af widgetten.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Returner HTML, der skal tilføjes i toppen af kommentarelementet.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Returner HTML, der skal tilføjes i bunden af kommentarelementet.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Returner HTML, der skal tilføjes efter kommentarteksten, inde i kommentarens indholdselement (bruges af afstemninger).
 * @property {Function} [replyAreaInputBottom] - Returner HTML, der skal tilføjes inde i kommentarens inputramme, under tekstinputtet (bruges af afstemninger til den indbyggede afstemningseditor). Modtager den overordnede kommentars id, eller null for roden svarboks.
 * @property {Function} [onPollUpdate] - Kaldes med live-eventet, når stemmetallene for en afstemning på siden ændrer sig.
 * @property {Function} isSiteAdmin - Returnerer, om seeren er admin eller moderator for tenanten. Kendes efter det første fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Returner HTML, der skal tilføjes i bunden af menuelementet for hver kommentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Rod-elementet.
 * @param {Object.<string, Function>} clickListeners - Eventhandlerne for klik, efter klassenavn, som kan ændres ved reference.
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