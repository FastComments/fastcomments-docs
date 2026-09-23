The extension object consists of the following definition:

<!-- als je dit wilt bijwerken, vergeet dan niet comment-ui-core bij te werken -->
[inline-code-attrs-start title = 'Extensieobject JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Het FastCommentsUI extensie‑object. Gebruikt voor lazy‑loading van bepaalde componenten. Bijvoorbeeld, het beoordelingssysteem wordt niet door alle klanten gebruikt, dus we laden die extensie alleen wanneer we deze nodig hebben.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - De widget root DOM‑knooppunt.
 * @property {string} [css]
 * @property {Object} config - Het FastComments‑configuratie‑object.
 * @property {Object} commentsById - Een verwijzing naar een object met alle reacties per id, dat up‑to‑date wordt gehouden.
 * @property {Object} translations - Een verwijzing naar alle vertalingen.
 * @property {Function} reRenderComment - Een verwijzing naar een functie die kan worden aangeroepen om een reactie opnieuw te renderen.
 * @property {Function} removeCommentAndReRender - Een verwijzing naar een functie die kan worden aangeroepen om een reactie uit het geheugen te verwijderen en het juiste deel van de DOM opnieuw te renderen.
 * @property {Function} newBroadcastId - Een verwijzing naar een functie die kan worden aangeroepen om een nieuw broadcast‑id te maken en toe te voegen aan de lokale lijst van te negeren broadcast‑ids.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Wordt aangeroepen met de reactie die geplaatst moet worden. Retourneer false om de inzending te annuleren (bijvoorbeeld wanneer een bijgevoegde poll onvolledig is).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filter HTML voor het reactie‑gebied.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filter HTML voor de volledige widget bij het renderen.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filter HTML voor elke reactie vóór het renderen.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filter HTML voor elk reactie‑menu vóór het renderen.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filter HTML voor de volledige widget bij het renderen.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (VEROUDERD) Retourneer HTML om toe te voegen aan de bovenkant van het reactie‑gebied.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (VEROUDERD) Retourneer HTML om toe te voegen aan de bovenkant van de widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (VEROUDERD) Retourneer HTML om toe te voegen aan de bovenkant van het reactie‑element.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (VEROUDERD) Retourneer HTML om toe te voegen aan de onderkant van het reactie‑element.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Retourneer HTML om toe te voegen na de reactietekst, binnen het reactie‑inhoudselement (gebruikt door polls).
 * @property {Function} [replyAreaInputBottom] - Retourneer HTML om toe te voegen binnen het invoer‑frame van de reactie, onder de tekstinvoer (gebruikt door polls voor de inline poll‑editor). Ontvangt de bovenliggende reactie‑id, of null voor het hoofd‑reactie‑vak.
 * @property {Function} [onPollUpdate] - Wordt aangeroepen met het live‑event wanneer de stemmingsaantallen van een poll op de pagina veranderen.
 * @property {Function} isSiteAdmin - Retourneert of de kijker een beheerder of moderator van de tenant is. Bekend na de eerste fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (VEROUDERD) Retourneer HTML om toe te voegen aan de onderkant van het menu‑element voor elke reactie.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Het root‑element.
 * @param {Object.<string, Function>} clickListeners - De event‑handlers voor klikken, per klassenaam, die via referentie kunnen worden aangepast.
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