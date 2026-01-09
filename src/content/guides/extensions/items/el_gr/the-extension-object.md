Το αντικείμενο επέκτασης αποτελείται από τον ακόλουθο ορισμό:

<!-- εάν θέλετε να ενημερώσετε αυτό, θυμηθείτε να ενημερώσετε το comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc αντικειμένου επέκτασης'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Ο ριζικός κόμβος DOM του widget.
 * @property {string} [css]
 * @property {Object} config - Το αντικείμενο ρυθμίσεων (config) του FastComments.
 * @property {Object} commentsById - Αναφορά σε ένα αντικείμενο με όλα τα σχόλια κατά id, το οποίο διατηρείται ενημερωμένο.
 * @property {Object} translations - Αναφορά σε όλες τις μεταφράσεις.
 * @property {Function} reRenderComment - Αναφορά σε μια συνάρτηση που μπορεί να κληθεί για να επανα-αποδώσει (re-render) ένα σχόλιο.
 * @property {Function} removeCommentAndReRender - Αναφορά σε μια συνάρτηση που μπορεί να κληθεί για να αφαιρέσει ένα σχόλιο από τη μνήμη και να επανα-αποδώσει το κατάλληλο μέρος του DOM.
 * @property {Function} newBroadcastId - Αναφορά σε μια συνάρτηση που μπορεί να δημιουργήσει ένα νέο broadcast id και να το προσθέσει στη τοπική λίστα των broadcast ids που θα αγνοηθούν.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Φιλτράρει το HTML για την περιοχή απάντησης (reply area).
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά το render.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Φιλτράρει το HTML για κάθε σχόλιο πριν από το render.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Φιλτράρει το HTML για κάθε μενού σχολίου πριν από το render.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά το render.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Επιστρέφει HTML για προσθήκη στο πάνω μέρος της περιοχής απάντησης.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Επιστρέφει HTML για προσθήκη στο πάνω μέρος του widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Επιστρέφει HTML για προσθήκη στο πάνω μέρος του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Επιστρέφει HTML για προσθήκη στο κάτω μέρος του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Επιστρέφει HTML για προσθήκη στο κάτω μέρος του στοιχείου μενού για κάθε σχόλιο.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Το ριζικό στοιχείο.
 * @param {Object.<string, Function>} clickListeners - Οι χειριστές γεγονότων για κλικς, κατά όνομα κλάσης, οι οποίοι μπορούν να τροποποιηθούν με αναφορά.
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