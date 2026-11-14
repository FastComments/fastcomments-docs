אובייקט ההרחבה מורכב מההגדרה הבאה:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'אובייקט ההרחבה JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * אובייקט ההרחבה של FastCommentsUI. משמש לטעינה עצלה (lazy-loading) של רכיבים מסוימים. לדוגמה, מערכת הביקורות אינה
 * בשימוש אצל כל הלקוחות, ולכן אנו טוענים את ההרחבה הזו רק כאשר אנו זקוקים לה.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - צומת ה-DOM הראשי של הווידג'ט.
 * @property {string} [css]
 * @property {Object} config - אובייקט התצורה של FastComments.
 * @property {Object} commentsById - הפניה לאובייקט המכיל את כל התגובות לפי מזהה, המתעדכן באופן שוטף.
 * @property {Object} translations - הפניה לכל התרגומים.
 * @property {Function} reRenderComment - הפניה לפונקציה שניתן להפעיל כדי לבצע רינדור מחדש של תגובה.
 * @property {Function} removeCommentAndReRender - הפניה לפונקציה שניתן להפעיל כדי להסיר תגובה מהזיכרון ולבצע רינדור מחדש של החלק המתאים ב-DOM.
 * @property {Function} newBroadcastId - הפניה לפונקציה שניתן להפעיל כדי ליצור מזהה שידור (broadcast id) חדש ולהוסיף אותו לרשימה המקומית של מזהי השידור שיש להתעלם מהם.
 * @property {Function} [withSessionId] - נקראת עם כתובת URL של בקשה ופונקציית callback. מפעילה את ה-callback עם ה-URL ומזהה הסשן של הצופה מצורף כאשר עוגיות צד שלישי נחסמות (אותו נתיב שבו משתמשות בקשות התגובה וההצבעה), או עם ה-URL ללא שינוי אחרת. אינה בשימוש עבור SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - נקראת עם התגובה העומדת להתפרסם. יש להחזיר false כדי לבטל את השליחה (למשל כאשר סקר מצורף אינו הושלם).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - סינון HTML עבור אזור התגובות.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - סינון HTML עבור הווידג'ט כולו בזמן רינדור.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - סינון HTML עבור כל תגובה לפני רינדור.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - סינון HTML עבור תפריט של כל תגובה לפני רינדור.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - סינון HTML עבור הווידג'ט כולו בזמן רינדור.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) מחזירה HTML להוספה בראש אזור התשובות.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) מחזירה HTML להוספה בראש הווידג'ט.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) מחזירה HTML להוספה בראש אלמנט התגובה.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) מחזירה HTML להוספה בתחתית אלמנט התגובה.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - מחזירה HTML להוספה אחרי טקסט התגובה, בתוך אלמנט תוכן התגובה (בשימוש על ידי סקרים).
 * @property {Function} [replyAreaInputBottom] - מחזירה HTML להוספה בתוך מסגרת קלט התגובה, מתחת לשדה הטקסט (בשימוש על ידי סקרים עבור עורך הסקר המובנה). מקבלת את מזהה תגובת האב, או null עבור תיבת התשובה הראשית.
 * @property {Function} [onPollUpdate] - נקראת עם האירוע החי כאשר מספרי ההצבעות של סקר בדף משתנים.
 * @property {Function} isSiteAdmin - מחזירה האם הצופה הוא מנהל או מפקח של הטננט. ידוע לאחר הבקשה הראשונה.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) מחזירה HTML להוספה בתחתית אלמנט התפריט עבור כל תגובה.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - האלמנט הראשי.
 * @param {Object.<string, Function>} clickListeners - מטפלי האירועים עבור לחיצות, לפי שם מחלקה, שניתן לשנותם באמצעות הפניה.
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