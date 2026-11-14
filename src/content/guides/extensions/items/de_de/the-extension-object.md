Das Extension-Objekt besteht aus der folgenden Definition:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Extension-Objekt JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Das FastCommentsUI Extension-Objekt. Wird zum Lazy-Loading bestimmter Komponenten verwendet. Zum Beispiel wird das
 * Bewertungssystem nicht von allen Kunden genutzt, daher laden wir diese Extension nur, wenn wir sie benötigen.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Der DOM-Wurzelknoten des Widgets.
 * @property {string} [css]
 * @property {Object} config - Das FastComments config-Objekt.
 * @property {Object} commentsById - Eine Referenz auf ein Objekt mit allen Kommentaren nach ID, das aktuell gehalten wird.
 * @property {Object} translations - Eine Referenz auf alle Übersetzungen.
 * @property {Function} reRenderComment - Eine Referenz auf eine Funktion, die aufgerufen werden kann, um einen Kommentar neu zu rendern.
 * @property {Function} removeCommentAndReRender - Eine Referenz auf eine Funktion, die aufgerufen werden kann, um einen Kommentar aus dem Speicher zu entfernen und den entsprechenden Teil des DOM neu zu rendern.
 * @property {Function} newBroadcastId - Eine Referenz auf eine Funktion, die aufgerufen werden kann, um eine neue Broadcast-ID zu erstellen und sie zur lokalen Liste der zu ignorierenden Broadcast-IDs hinzuzufügen.
 * @property {Function} [withSessionId] - Aufruf mit einer Request-URL und einem Callback. Ruft den Callback mit der URL plus der angehängten Session-ID des Betrachters auf, wenn Third-Party-Cookies blockiert werden (derselbe Pfad, den Kommentar- und Vote-Requests verwenden), andernfalls mit der unveränderten URL. Wird nicht für SSO verwendet.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Wird mit dem Kommentar aufgerufen, der gerade gepostet werden soll. Gib false zurück, um das Absenden abzubrechen (z. B. wenn eine angehängte Umfrage unvollständig ist).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtert HTML für den Kommentarbereich.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtert HTML für das gesamte Widget beim Rendern.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtert HTML für jeden Kommentar vor dem Rendern.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtert HTML für das Menü jedes Kommentars vor dem Rendern.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtert HTML für das gesamte Widget beim Rendern.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Gibt HTML zurück, das am oberen Rand des Antwortbereichs hinzugefügt wird.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Gibt HTML zurück, das am oberen Rand des Widgets hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Gibt HTML zurück, das am oberen Rand des Kommentar-Elements hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Gibt HTML zurück, das am unteren Rand des Kommentar-Elements hinzugefügt wird.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Gibt HTML zurück, das nach dem Kommentartext innerhalb des Inhalts-Elements des Kommentars hinzugefügt wird (von Umfragen verwendet).
 * @property {Function} [replyAreaInputBottom] - Gibt HTML zurück, das innerhalb des Rahmens des Kommentar-Eingabefelds unterhalb der Texteingabe hinzugefügt wird (von Umfragen für den Inline-Umfrage-Editor verwendet). Erhält die ID des übergeordneten Kommentars oder null für das Antwortfeld auf der obersten Ebene.
 * @property {Function} [onPollUpdate] - Wird mit dem Live-Event aufgerufen, wenn sich die Stimmenzahlen einer Umfrage auf der Seite ändern.
 * @property {Function} isSiteAdmin - Gibt zurück, ob der Betrachter ein Admin oder Moderator des Tenants ist. Erst nach dem ersten Abruf bekannt.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Gibt HTML zurück, das am unteren Rand des Menü-Elements für jeden Kommentar hinzugefügt wird.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Das Wurzelelement.
 * @param {Object.<string, Function>} clickListeners - Die Event-Handler für Klicks, nach Klassenname, die per Referenz geändert werden können.
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