---
Het extensieobject bestaat uit de volgende definitie:

<!-- als je dit wilt bijwerken, vergeet dan niet comment-ui-core bij te werken -->
[inline-code-attrs-start title = 'JSDoc van Extensieobject'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Het FastCommentsUI-extensieobject. Wordt gebruikt om bepaalde componenten lazy te laden. Bijvoorbeeld, het reviewsysteem wordt niet
 * door alle klanten gebruikt, dus laden we die extensie alleen wanneer we die nodig hebben.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - De root DOM-node van de widget.
 * @property {string} [css]
 * @property {Object} config - Het FastComments-configuratieobject.
 * @property {Object} commentsById - Een referentie naar een object met alle reacties per id, dat up-to-date wordt gehouden.
 * @property {Object} translations - Een referentie naar alle vertalingen.
 * @property {Function} reRenderComment - Een referentie naar een functie die aangeroepen kan worden om een reactie opnieuw te renderen.
 * @property {Function} removeCommentAndReRender - Een referentie naar een functie die aangeroepen kan worden om een reactie uit het geheugen te verwijderen en het juiste deel van de DOM opnieuw te renderen.
 * @property {Function} newBroadcastId - Een referentie naar een functie die kan worden aangeroepen om een nieuwe broadcast-id te maken en toe te voegen aan de lokale lijst met te negeren broadcast-id's.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filter HTML voor het reactiegebied.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filter HTML voor de hele widget bij het renderen.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filter HTML voor elke reactie vóór het renderen.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filter HTML voor elk reactiemenu vóór het renderen.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filter HTML voor de hele widget bij het renderen.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Retourneer HTML die bovenaan het reactiegebied wordt toegevoegd.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Retourneer HTML die bovenaan de widget wordt toegevoegd.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Retourneer HTML die bovenaan het reactie-element wordt toegevoegd.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Retourneer HTML die onderaan het reactie-element wordt toegevoegd.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Retourneer HTML die onderaan het menuelement voor elke reactie wordt toegevoegd.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Het root-element.
 * @param {Object.<string, Function>} clickListeners - De eventhandlers voor klikgebeurtenissen, gegroepeerd per classnaam, die via referentie kunnen worden aangepast.
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


---