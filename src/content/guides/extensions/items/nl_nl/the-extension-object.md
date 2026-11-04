Het extension-object bestaat uit de volgende definitie:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Extension Object JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Het FastCommentsUI extension-object. Wordt gebruikt voor het lazy-loaden van bepaalde componenten. Zo wordt het
 * reviewsysteem bijvoorbeeld niet door alle klanten gebruikt, dus laden we die extensie alleen wanneer we die nodig hebben.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Het root DOM-node van de widget.
 * @property {string} [css]
 * @property {Object} config - Het FastComments config-object.
 * @property {Object} commentsById - Een verwijzing naar een object met alle reacties op id, dat actueel wordt gehouden.
 * @property {Object} translations - Een verwijzing naar alle vertalingen.
 * @property {Function} reRenderComment - Een verwijzing naar een functie die kan worden aangeroepen om een reactie opnieuw te renderen.
 * @property {Function} removeCommentAndReRender - Een verwijzing naar een functie die kan worden aangeroepen om een reactie uit het geheugen te verwijderen en het betreffende deel van de DOM opnieuw te renderen.
 * @property {Function} newBroadcastId - Een verwijzing naar een functie die kan worden aangeroepen om een nieuw broadcast id aan te maken en toe te voegen aan de lokale lijst met te negeren broadcast ids.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Wordt aangeroepen met de reactie die geplaatst gaat worden. Geef false terug om het verzenden te annuleren (bijvoorbeeld wanneer een bijgevoegde poll onvolledig is).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtert HTML voor het reactiegebied.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtert HTML voor de hele widget bij het renderen.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtert HTML voor elke reactie vóór het renderen.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtert HTML voor het menu van elke reactie vóór het renderen.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtert HTML voor de hele widget bij het renderen.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Geef HTML terug om bovenaan het reactiegebied toe te voegen.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Geef HTML terug om bovenaan de widget toe te voegen.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Geef HTML terug om bovenaan het reactie-element toe te voegen.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Geef HTML terug om onderaan het reactie-element toe te voegen.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Geef HTML terug om na de reactietekst toe te voegen, binnen het content-element van de reactie (gebruikt door polls).
 * @property {Function} [replyAreaInputBottom] - Geef HTML terug om binnen het invoerkader van de reactie toe te voegen, onder de tekstinvoer (gebruikt door polls voor de in-place poll-editor). Ontvangt de id van de bovenliggende reactie, of null voor het hoofdreactievak.
 * @property {Function} [onPollUpdate] - Wordt aangeroepen met het live event wanneer de stemtellingen van een poll op de pagina veranderen.
 * @property {Function} isSiteAdmin - Geeft terug of de kijker een admin of moderator van de tenant is. Bekend na de eerste fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Geef HTML terug om onderaan het menu-element van elke reactie toe te voegen.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Het root-element.
 * @param {Object.<string, Function>} clickListeners - De event handlers voor klikken, op klassenaam, die by reference gewijzigd kunnen worden.
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