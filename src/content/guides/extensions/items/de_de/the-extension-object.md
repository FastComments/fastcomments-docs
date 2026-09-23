The extension object consists of the following definition:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Erweiterungsobjekt JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Das FastCommentsUI-Erweiterungsobjekt. Wird zum Lazy-Loading bestimmter Komponenten verwendet. Zum Beispiel wird das Bewertungssystem nicht von allen Kunden genutzt, sodass wir diese Erweiterung nur laden, wenn wir sie benötigen.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Der Wurzel-DOM-Knoten des Widgets.
 * @property {string} [css]
 * @property {Object} config - Das FastComments-Konfigurationsobjekt.
 * @property {Object} commentsById - Ein Verweis auf ein Objekt mit allen Kommentaren nach ID, das stets aktuell gehalten wird.
 * @property {Object} translations - Ein Verweis auf alle Übersetzungen.
 * @property {Function} reRenderComment - Ein Verweis auf eine Funktion, die aufgerufen werden kann, um einen Kommentar neu zu rendern.
 * @property {Function} removeCommentAndReRender - Ein Verweis auf eine Funktion, die aufgerufen werden kann, um einen Kommentar aus dem Speicher zu entfernen und den entsprechenden Teil des DOM neu zu rendern.
 * @property {Function} newBroadcastId - Ein Verweis auf eine Funktion, die aufgerufen werden kann, um eine neue Broadcast-ID zu erstellen und sie zur lokalen Liste der zu ignorierenden Broadcast-IDs hinzuzufügen.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Wird mit dem Kommentar aufgerufen, der gepostet werden soll. Gibt false zurück, um das Absenden abzubrechen (z. B. wenn eine angehängte Umfrage unvollständig ist).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtert HTML für den Kommentarbereich.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtert HTML für das gesamte Widget beim Rendern.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtert HTML für jeden Kommentar vor dem Rendern.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtert HTML für jedes Kommentarmenü vor dem Rendern.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtert HTML für das gesamte Widget beim Rendern.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (VERALTET) Gibt HTML zurück, das oben im Antwortbereich hinzugefügt wird.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (VERALTET) Gibt HTML zurück, das oben im Widget hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (VERALTET) Gibt HTML zurück, das oben im Kommentar-Element hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (VERALTET) Gibt HTML zurück, das unten im Kommentar-Element hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Gibt HTML zurück, das nach dem Kommentartext im Kommentar-Inhaltselement hinzugefügt wird (verwendet von Umfragen).
 * @property {Function} [replyAreaInputBottom] - Gibt HTML zurück, das im Kommentar-Eingabefeld unterhalb des Texteingabefeldes hinzugefügt wird (verwendet von Umfragen für den In-Place-Umfrage-Editor). Erhält die übergeordnete Kommentar-ID oder null für das Hauptantwortfeld.
 * @property {Function} [onPollUpdate] - Wird mit dem Live-Event aufgerufen, wenn sich die Stimmenzahlen einer Umfrage auf der Seite ändern.
 * @property {Function} isSiteAdmin - Gibt zurück, ob der Betrachter ein Administrator oder Moderator des Mandanten ist. Bekannt nach dem ersten Abruf.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (VERALTET) Gibt HTML zurück, das unten im Menü-Element für jeden Kommentar hinzugefügt wird.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Das Wurzelelement.
 * @param {Object.<string, Function>} clickListeners - Die Ereignis-Handler für Klicks, nach Klassenname, die per Referenz modifiziert werden können.
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