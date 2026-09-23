Udvidelsesobjektet består af følgende definition:

<!-- Hvis du vil opdatere dette, så husk at opdatere comment-ui-core -->
[inline-code-attrs-start title = 'Udvidelsesobjekt JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI‑udvidelsesobjektet. Bruges til lazy‑loading af visse komponenter. For eksempel bruges anmeldelsessystemet ikke
 * af alle kunder, så vi indlæser kun den udvidelse, når vi har brug for den.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Widgetens rod‑DOM‑node.
 * @property {string} [css]
 * @property {Object} config - FastComments konfigurationsobjekt.
 * @property {Object} commentsById - En reference til et objekt med alle kommentarer efter id, som holdes opdateret.
 * @property {Object} translations - En reference til alle oversættelser.
 * @property {Function} reRenderComment - En reference til en funktion, der kan påkaldes for at genrendere en kommentar.
 * @property {Function} removeCommentAndReRender - En reference til en funktion, der kan påkaldes for at fjerne en kommentar fra hukommelsen og genrendere den relevante del af DOM'en.
 * @property {Function} newBroadcastId - En reference til en funktion, der kan påkaldes for at oprette et nyt broadcast‑id og tilføje det til den lokale liste over broadcast‑id'er, der skal ignoreres.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Kaldes med kommentaren, der skal postes. Returner false for at annullere indsendelsen (for eksempel når en vedhæftet afstemning er ufuldstændig).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrer HTML for kommentarområdet.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrer HTML for hele widgeten ved rendering.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrer HTML for hver kommentar før rendering.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrer HTML for hver kommentarmenu før rendering.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrer HTML for hele widgeten ved rendering.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Returner HTML, der skal tilføjes til toppen af svarområdet.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Returner HTML, der skal tilføjes til toppen af widgeten.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Returner HTML, der skal tilføjes til toppen af kommentar‑elementet.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Returner HTML, der skal tilføjes til bunden af kommentar‑elementet.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Returner HTML, der skal tilføjes efter kommentarteksten, inde i kommentarindholdselementet (bruges af afstemninger).
 * @property {Function} [replyAreaInputBottom] - Returner HTML, der skal tilføjes inde i kommentarinputrammen, under tekstinput (bruges af afstemninger til den indlejrede afstemningseditor). Modtager den overordnede kommentar‑id, eller null for rodrækkens svarboks.
 * @property {Function} [onPollUpdate] - Kaldes med live‑begivenheden, når stemmetalene for en afstemning på siden ændres.
 * @property {Function} isSiteAdmin - Returnerer om seeren er en admin eller moderator af lejer. Kendt efter den første hentning.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Returner HTML, der skal tilføjes til bunden af menuelementet for hver kommentar.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - The root element.
 * @param {Object.<string, Function>} clickListeners - The event handlers for clicks, by class name, which can be modified by reference.
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