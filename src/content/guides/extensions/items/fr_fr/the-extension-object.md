L'objet d'extension se compose de la définition suivante :

<!-- si vous voulez mettre à jour ceci, n'oubliez pas de mettre à jour comment-ui-core -->
[inline-code-attrs-start title = "JSDoc de l'objet d'extension"; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - The widget root dom node.
 * @property {string} [css]
 * @property {Object} config - The FastComments config object.
 * @property {Object} commentsById - A reference to an object with all comments by id, which is kept up to date.
 * @property {Object} translations - A reference to all translations.
 * @property {Function} reRenderComment - A reference to a function that can be invoked to re-render a comment.
 * @property {Function} removeCommentAndReRender - A reference to a function that can be invoked to remove a comment from memory and re-render the appropriate part of the DOM.
 * @property {Function} newBroadcastId - A reference to a function that can be invoked create a new broadcast id and add it to the local list of broadcast ids to ignore.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filter HTML for the comment area.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filter HTML for the whole widget on render.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filter HTML for each comment before render.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filter HTML for each comment menu before render.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filter HTML for the whole widget on render.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Return HTML to add to the top of the reply area.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Return HTML to add to the top of the widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Return HTML to add to the top of the comment element.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Return HTML to add to the bottom of the comment element.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Return HTML to add to the bottom of the menu element for each comment.
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