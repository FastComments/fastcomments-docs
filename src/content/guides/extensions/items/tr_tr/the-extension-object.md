Eklenti nesnesi aşağıdaki tanımdan oluşur:

<!-- eğer bunu güncellemek istiyorsanız, comment-ui-core'u güncellemeyi unutmayın -->
[inline-code-attrs-start title = 'Eklenti Nesnesi JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Widget kök DOM düğümü.
 * @property {string} [css]
 * @property {Object} config - FastComments yapılandırma nesnesi.
 * @property {Object} commentsById - id'ye göre tüm yorumları içeren bir nesneye referans; güncel tutulur.
 * @property {Object} translations - tüm çevirilere bir referans.
 * @property {Function} reRenderComment - bir yorumu yeniden render etmek için çağrılabilecek bir fonksiyona referans.
 * @property {Function} removeCommentAndReRender - bir yorumu bellekte kaldırıp DOM'un uygun bölümünü yeniden render etmek için çağrılabilecek bir fonksiyona referans.
 * @property {Function} newBroadcastId - yeni bir yayın id'si oluşturup yerel yok sayılacak yayın id'leri listesine eklemek için çağrılabilecek bir fonksiyona referans.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Yorum alanı için HTML'i filtreler.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Render sırasında tüm widget için HTML'i filtreler.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Render öncesinde her yorum için HTML'i filtreler.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Render öncesinde her yorum menüsü için HTML'i filtreler.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Render sırasında tüm widget için HTML'i filtreler.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ESKİ) Yanıt alanının üstüne eklemek için HTML döndürür.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ESKİ) Widget'in üstüne eklemek için HTML döndürür.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ESKİ) Yorum elemanının üstüne eklemek için HTML döndürür.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ESKİ) Yorum elemanının altına eklemek için HTML döndürür.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ESKİ) Her yorum için menü elemanının altına eklemek üzere HTML döndürür.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Kök element.
 * @param {Object.<string, Function>} clickListeners - Sınıf adına göre tıklama olay işleyicileri; referans olarak değiştirilebilir.
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