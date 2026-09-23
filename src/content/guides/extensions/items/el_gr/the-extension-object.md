Το αντικείμενο επέκτασης αποτελείται από τον ακόλουθο ορισμό:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Αντικείμενο Επέκτασης JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * Το αντικείμενο επέκτασης FastCommentsUI. Χρησιμοποιείται για lazy-loading ορισμένων στοιχείων. Για παράδειγμα, το σύστημα αξιολογήσεων δεν
 * χρησιμοποιείται από όλους τους πελάτες, έτσι φορτώνουμε αυτήν την επέκταση μόνο όταν τη χρειαζόμαστε.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Ο ριζικός κόμβος DOM του widget.
 * @property {string} [css]
 * @property {Object} config - Το αντικείμενο ρυθμίσεων FastComments.
 * @property {Object} commentsById - Μια αναφορά σε ένα αντικείμενο με όλα τα σχόλια κατά id, το οποίο διατηρείται ενημερωμένο.
 * @property {Object} translations - Μια αναφορά σε όλες τις μεταφράσεις.
 * @property {Function} reRenderComment - Μια αναφορά σε μια συνάρτηση που μπορεί να κληθεί για επανασχεδίαση ενός σχολίου.
 * @property {Function} removeCommentAndReRender - Μια αναφορά σε μια συνάρτηση που μπορεί να κληθεί για αφαίρεση ενός σχολίου από τη μνήμη και επανασχεδίαση του κατάλληλου τμήματος του DOM.
 * @property {Function} newBroadcastId - Μια αναφορά σε μια συνάρτηση που μπορεί να κληθεί για δημιουργία νέου broadcast id και προσθήκη του στη τοπική λίστα των broadcast ids προς παράβλεψη.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Καλείται με το σχόλιο που πρόκειται να δημοσιευθεί. Επιστρέφει false για ακύρωση της υποβολής (π.χ. όταν μια συνημμένη δημοσκόπηση είναι ημιτελής).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Φιλτράρει το HTML για την περιοχή σχολίων.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά την απόδοση.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Φιλτράρει το HTML για κάθε σχόλιο πριν την απόδοση.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Φιλτράρει το HTML για κάθε μενού σχολίου πριν την απόδοση.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Φιλτράρει το HTML για ολόκληρο το widget κατά την απόδοση.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ΠΑΡΑΚΑΤΑΓΩΓΗ) Επιστρέφει HTML για προσθήκη στην κορυφή της περιοχής απαντήσεων.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ΠΑΡΑΚΑΤΑΓΩΓΗ) Επιστρέφει HTML για προσθήκη στην κορυφή του widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ΠΑΡΑΚΑΤΑΓΩΓΗ) Επιστρέφει HTML για προσθήκη στην κορυφή του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ΠΑΡΑΚΑΤΑΓΩΓΗ) Επιστρέφει HTML για προσθήκη στο κάτω μέρος του στοιχείου σχολίου.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Επιστρέφει HTML για προσθήκη μετά το κείμενο του σχολίου, μέσα στο στοιχείο περιεχομένου σχολίου (χρησιμοποιείται από δημοσκοπήσεις).
 * @property {Function} [replyAreaInputBottom] - Επιστρέφει HTML για προσθήκη μέσα στο πλαίσιο εισαγωγής σχολίου, κάτω από το πεδίο κειμένου (χρησιμοποιείται από δημοσκοπήσεις για τον ενσωματωμένο επεξεργαστή δημοσκόπησης). Λαμβάνει το id του γονικού σχολίου ή null για το ριζικό πλαίσιο απάντησης.
 * @property {Function} [onPollUpdate] - Καλείται με το ζωντανό γεγονός όταν αλλάζουν οι μετρήσεις ψήφων μιας δημοσκόπησης στη σελίδα.
 * @property {Function} isSiteAdmin - Επιστρέφει αν ο θεατής είναι διαχειριστής ή συντονιστής του ενοικιαστή. Γνωρίζεται μετά το πρώτο fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ΠΑΡΑΚΑΤΑΓΩΓΗ) Επιστρέφει HTML για προσθήκη στο κάτω μέρος του στοιχείου μενού για κάθε σχόλιο.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Το ριζικό στοιχείο.
 * @param {Object.<string, Function>} clickListeners - Οι χειριστές συμβάντων για κλικ, κατά όνομα κλάσης, οι οποίοι μπορούν να τροποποιηθούν με αναφορά.
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