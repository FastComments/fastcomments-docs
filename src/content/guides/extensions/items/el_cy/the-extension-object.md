Το αντικείμενο επέκτασης αποτελείται από τον ακόλουθο ορισμό:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Αντικείμενο Επέκτασης JSDoc'; type = 'javascript'; inline-code-attrs-end]
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
 * @property {Object} commentsById - Αναφορά σε αντικείμενο με όλα τα σχόλια ανά id, το οποίο διατηρείται ενημερωμένο.
 * @property {Object} translations - Αναφορά σε όλες τις μεταφράσεις.
 * @property {Function} reRenderComment - Αναφορά σε συνάρτηση που μπορεί να κληθεί για να επανα-αποδώσει (re-render) ένα σχόλιο.
 * @property {Function} removeCommentAndReRender - Αναφορά σε συνάρτηση που μπορεί να κληθεί για να αφαιρέσει ένα σχόλιο από τη μνήμη και να επανα-αποδώσει το αντίστοιχο μέρος του DOM.
 * @property {Function} newBroadcastId - Αναφορά σε συνάρτηση που μπορεί να κληθεί για να δημιουργήσει ένα νέο broadcast id και να το προσθέσει στην τοπική λίστα broadcast ids που θα αγνοούνται.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Φιλτράρει το HTML για την περιοχή σχολίων.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά την απόδοση (render).
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Φιλτράρει το HTML για κάθε σχόλιο πριν από την απόδοση.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Φιλτράρει το HTML για κάθε μενού σχολίου πριν από την απόδοση.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά την απόδοση (render).
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ΠΑΛΑΙΟ) Επιστρέφει HTML για προσθήκη στην κορυφή της περιοχής απαντήσεων.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ΠΑΛΑΙΟ) Επιστρέφει HTML για προσθήκη στην κορυφή του widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ΠΑΛΑΙΟ) Επιστρέφει HTML για προσθήκη στην κορυφή του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ΠΑΛΑΙΟ) Επιστρέφει HTML για προσθήκη στο κάτω μέρος του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ΠΑΛΑΙΟ) Επιστρέφει HTML για προσθήκη στο κάτω μέρος του στοιχείου μενού για κάθε σχόλιο.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Το ριζικό στοιχείο.
 * @param {Object.<string, Function>} clickListeners - Οι χειριστές γεγονότων για κλικ, κατά όνομα κλάσης, οι οποίοι μπορούν να τροποποιηθούν κατά αναφορά.
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