Το αντικείμενο extension αποτελείται από τον παρακάτω ορισμό:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc του αντικειμένου Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Το αντικείμενο extension του FastCommentsUI. Χρησιμοποιείται για lazy-loading ορισμένων components. Για παράδειγμα, το σύστημα κριτικών δεν
 * χρησιμοποιείται από όλους τους πελάτες, οπότε φορτώνουμε αυτό το extension μόνο όταν το χρειαζόμαστε.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Ο ριζικός DOM κόμβος του widget.
 * @property {string} [css]
 * @property {Object} config - Το αντικείμενο config του FastComments.
 * @property {Object} commentsById - Αναφορά σε αντικείμενο με όλα τα σχόλια ανά id, το οποίο διατηρείται ενημερωμένο.
 * @property {Object} translations - Αναφορά σε όλες τις μεταφράσεις.
 * @property {Function} reRenderComment - Αναφορά σε συνάρτηση που μπορεί να κληθεί για να επανεμφανίσει ένα σχόλιο.
 * @property {Function} removeCommentAndReRender - Αναφορά σε συνάρτηση που μπορεί να κληθεί για να αφαιρέσει ένα σχόλιο από τη μνήμη και να επανεμφανίσει το κατάλληλο τμήμα του DOM.
 * @property {Function} newBroadcastId - Αναφορά σε συνάρτηση που μπορεί να κληθεί για να δημιουργήσει ένα νέο broadcast id και να το προσθέσει στην τοπική λίστα των broadcast ids προς αγνόηση.
 * @property {Function} [withSessionId] - Καλείται με ένα URL αιτήματος και ένα callback. Καλεί το callback με το URL συν το session id του θεατή προσαρτημένο όταν τα third-party cookies είναι μπλοκαρισμένα (ίδια διαδρομή που χρησιμοποιούν τα αιτήματα σχολίων και ψήφων), ή το URL αμετάβλητο διαφορετικά. Δεν χρησιμοποιείται για SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Καλείται με το σχόλιο που πρόκειται να δημοσιευτεί. Επιστρέψτε false για να ακυρώσετε την υποβολή (για παράδειγμα όταν μια συνημμένη ψηφοφορία είναι ελλιπής).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Φιλτράρει το HTML για την περιοχή σχολίων.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά την εμφάνιση.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Φιλτράρει το HTML για κάθε σχόλιο πριν την εμφάνιση.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Φιλτράρει το HTML για το μενού κάθε σχολίου πριν την εμφάνιση.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά την εμφάνιση.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Επιστρέφει HTML για προσθήκη στην κορυφή της περιοχής απάντησης.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Επιστρέφει HTML για προσθήκη στην κορυφή του widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Επιστρέφει HTML για προσθήκη στην κορυφή του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Επιστρέφει HTML για προσθήκη στο τέλος του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Επιστρέφει HTML για προσθήκη μετά το κείμενο του σχολίου, μέσα στο στοιχείο περιεχομένου του σχολίου (χρησιμοποιείται από ψηφοφορίες).
 * @property {Function} [replyAreaInputBottom] - Επιστρέφει HTML για προσθήκη μέσα στο πλαίσιο εισαγωγής σχολίου, κάτω από το πεδίο κειμένου (χρησιμοποιείται από ψηφοφορίες για τον επεξεργαστή ψηφοφορίας επί τόπου). Λαμβάνει το id του γονικού σχολίου, ή null για το κύριο πλαίσιο απάντησης.
 * @property {Function} [onPollUpdate] - Καλείται με το live event όταν αλλάζουν οι μετρητές ψήφων μιας ψηφοφορίας στη σελίδα.
 * @property {Function} isSiteAdmin - Επιστρέφει αν ο θεατής είναι διαχειριστής ή συντονιστής του tenant. Γίνεται γνωστό μετά το πρώτο fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Επιστρέφει HTML για προσθήκη στο τέλος του στοιχείου μενού για κάθε σχόλιο.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Το ριζικό στοιχείο.
 * @param {Object.<string, Function>} clickListeners - Οι handlers συμβάντων για κλικ, ανά όνομα κλάσης, οι οποίοι μπορούν να τροποποιηθούν κατ' αναφορά.
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