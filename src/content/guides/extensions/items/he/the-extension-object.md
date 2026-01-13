אובייקט התוסף מורכב מההגדרה הבאה:

<!-- אם אתם רוצים לעדכן זאת, זכרו לעדכן גם את comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc של אובייקט התוסף'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - צומת ה-DOM השורש של הווידג'ט.
 * @property {string} [css]
 * @property {Object} config - אובייקט התצורה של FastComments.
 * @property {Object} commentsById - מצביע לאובייקט שמכיל את כל התגובות לפי id, שמתעדכן באופן שוטף.
 * @property {Object} translations - הפניה לכל התרגומים.
 * @property {Function} reRenderComment - הפניה לפונקציה שניתן לקרוא לה כדי לרנדר מחדש תגובה.
 * @property {Function} removeCommentAndReRender - הפניה לפונקציה שניתן לקרוא לה כדי להסיר תגובה מהזיכרון ולרנדר מחדש את החלק המתאים ב-DOM.
 * @property {Function} newBroadcastId - הפניה לפונקציה שניתן לקרוא לה כדי ליצור מזהה שידור חדש ולהוסיף אותו לרשימה מקומית של מזהי שידור שיש להתעלם מהם.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - מסנן HTML לאזור התגובות.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - מסנן HTML לכל הווידג'ט בעת רינדור.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - מסנן HTML לכל תגובה לפני רינדור.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - מסנן HTML לתפריט של כל תגובה לפני רינדור.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - מסנן HTML לכל הווידג'ט בעת רינדור.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) מחזיר HTML להוספה לראש אזור התגובה.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) מחזיר HTML להוספה לראש הווידג'ט.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) מחזיר HTML להוספה לראש אלמנט התגובה.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) מחזיר HTML להוספה לתחתית אלמנט התגובה.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) מחזיר HTML להוספה לתחתית אלמנט התפריט לכל תגובה.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - האלמנט השורש.
 * @param {Object.<string, Function>} clickListeners - מטפלי אירועים ללחיצות, לפי שם מחלקה, שניתן לשנות על-ידי הפניה.
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