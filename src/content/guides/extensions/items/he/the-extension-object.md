[inline-code-attrs-start title = 'אובייקט הרחבה JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * אובייקט ההרחבה FastCommentsUI. משמש לטעינה עצלה של רכיבים מסוימים. לדוגמה, מערכת הסקירות אינה משומשת על ידי כל הלקוחות, ולכן אנו טוענים את ההרחבה רק כאשר אנו זקוקים לה.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - צומת ה‑DOM השורש של הווידג'ט.
 * @property {string} [css]
 * @property {Object} config - אובייקט התצורה של FastComments.
 * @property {Object} commentsById - הפנייה לאובייקט המכיל את כל ההערות לפי מזהה, המתעדכן באופן שוטף.
 * @property {Object} translations - הפנייה לכל התרגומים.
 * @property {Function} reRenderComment - הפנייה לפונקציה שניתן לקרוא לה כדי לעדכן מחדש תגובה.
 * @property {Function} removeCommentAndReRender - הפנייה לפונקציה שניתן לקרוא לה כדי להסיר תגובה מהזיכרון ולעדכן מחדש את החלק המתאים ב‑DOM.
 * @property {Function} newBroadcastId - הפנייה לפונקציה שניתן לקרוא לה כדי ליצור מזהה שידור חדש ולהוסיף אותו לרשימה המקומית של מזהי שידור שיש להתעלם מהם.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - נקראת עם התגובה שעומדת להישלח. מחזירה false כדי לבטל את השליחה (לדוגמה כאשר סקר מצורף אינו שלם).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - סינון HTML לאזור התגובה.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - סינון HTML לכל הווידג'ט בעת הרינדור.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - סינון HTML לכל תגובה לפני הרינדור.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - סינון HTML לכל תפריט תגובה לפני הרינדור.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - סינון HTML לכל הווידג'ט בעת הרינדור.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) מחזירה HTML להוספה לחלק העליון של אזור התגובה.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) מחזירה HTML להוספה לחלק העליון של הווידג'ט.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) מחזירה HTML להוספה לחלק העליון של אלמנט התגובה.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) מחזירה HTML להוספה לחלק התחתון של אלמנט התגובה.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - מחזירה HTML להוספה אחרי טקסט התגובה, בתוך אלמנט תוכן התגובה (משמש בסקרים).
 * @property {Function} [replyAreaInputBottom] - מחזירה HTML להוספה בתוך מסגרת קלט התגובה, מתחת לשדה הטקסט (משמש בסקרים עבור עורך סקר במקום). מקבלת את מזהה ההורה של התגובה, או null עבור תיבת התגובה השורשית.
 * @property {Function} [onPollUpdate] - נקראת עם האירוע החי כאשר ספירות הקולות של סקר בעמוד משתנות.
 * @property {Function} isSiteAdmin - מחזירה האם הצופה הוא מנהל או מודרטור של השוכרת. ידוע לאחר ההבאה הראשונה.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) מחזירה HTML להוספה לחלק התחתון של אלמנט התפריט עבור כל תגובה.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - אלמנט השורש.
 * @param {Object.<string, Function>} clickListeners - מאפייני אירועי לחיצה, לפי שם מחלקה, שניתן לשנות בהתייחסות.
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