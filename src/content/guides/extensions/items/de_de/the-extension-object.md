Das Erweiterungsobjekt besteht aus der folgenden Definition:

<!-- Wenn Sie dies aktualisieren möchten, denken Sie daran, comment-ui-core zu aktualisieren -->
[inline-code-attrs-start title = 'JSDoc des Erweiterungsobjekts'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Das Wurzel-DOM-Element des Widgets.
 * @property {string} [css]
 * @property {Object} config - Das FastComments-Konfigurationsobjekt.
 * @property {Object} commentsById - Eine Referenz auf ein Objekt mit allen Kommentaren nach ID, das aktuell gehalten wird.
 * @property {Object} translations - Eine Referenz auf alle Übersetzungen.
 * @property {Function} reRenderComment - Eine Referenz auf eine Funktion, die aufgerufen werden kann, um einen Kommentar neu zu rendern.
 * @property {Function} removeCommentAndReRender - Eine Referenz auf eine Funktion, die aufgerufen werden kann, um einen Kommentar aus dem Speicher zu entfernen und den entsprechenden Teil des DOM neu zu rendern.
 * @property {Function} newBroadcastId - Eine Referenz auf eine Funktion, die aufgerufen werden kann, um eine neue Broadcast-ID zu erstellen und sie zur lokalen Liste der zu ignorierenden Broadcast-IDs hinzuzufügen.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - HTML für den Antwortbereich filtern.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - HTML für das gesamte Widget beim Rendern filtern.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - HTML für jeden Kommentar vor dem Rendern filtern.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - HTML für jedes Kommentarmenü vor dem Rendern filtern.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - HTML für das gesamte Widget beim Rendern filtern.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (VERALTET) Gibt HTML zurück, das oben im Antwortbereich hinzugefügt wird.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (VERALTET) Gibt HTML zurück, das oben im Widget hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (VERALTET) Gibt HTML zurück, das oben im Kommentarelement hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (VERALTET) Gibt HTML zurück, das unten im Kommentarelement hinzugefügt wird.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (VERALTET) Gibt HTML zurück, das unten im Menuelement für jeden Kommentar hinzugefügt wird.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Das Wurzelelement.
 * @param {Object.<string, Function>} clickListeners - Die Event-Handler für Klicks, nach Klassenname, die per Referenz verändert werden können.
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