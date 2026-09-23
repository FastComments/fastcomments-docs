The extension object consists of the following definition:

<!-- eğer bunu güncellemek istiyorsanız, comment-ui-core'u güncellemeyi unutmayın -->
[inline-code-attrs-start title = 'Uzantı Nesnesi JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI uzantı nesnesi. Belirli bileşenlerin tembel yüklenmesi için kullanılır. Örneğin, inceleme sistemi tüm müşteriler tarafından kullanılmaz, bu yüzden bu uzantıyı yalnızca ihtiyacımız olduğunda yükleriz.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Widget'ın kök DOM düğümü.
 * @property {string} [css]
 * @property {Object} config - FastComments yapılandırma nesnesi.
 * @property {Object} commentsById - Tüm yorumların kimliğe göre bulunduğu bir nesneye referans, güncel tutulur.
 * @property {Object} translations - Tüm çevirilere bir referans.
 * @property {Function} reRenderComment - Bir yorumu yeniden renderlamak için çağrılabilecek bir fonksiyona referans.
 * @property {Function} removeCommentAndReRender - Yorum

u bellekten kaldırmak ve DOM'un ilgili kısmını yeniden renderlamak için çağrılabilecek bir fonksiyona referans.
 * @property {Function} newBroadcastId - Yeni bir yayın kimliği oluşturmak ve bunu yok sayılacak yerel yayın kimliği listesine eklemek için çağrılabilecek bir fonksiyona referans.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Gönderilmek üzere olan yorumla birlikte çağrılır. Gönderimi iptal etmek için false döndürün (örneğin ekli anket eksik olduğunda).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Yorum alanı için HTML'i filtrele.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Tüm widget'ın render sırasında HTML'ini filtrele.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Render öncesinde her yorum için HTML'i filtrele.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Render öncesinde her yorum menüsü için HTML'i filtrele.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Render sırasında tüm widget için HTML'i filtrele.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ESKİ) Yanıt alanının üstüne eklemek için HTML döndür.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ESKİ) Widget'ın üstüne eklemek için HTML döndür.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ESKİ) Yorum öğesinin üstüne eklemek için HTML döndür.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ESKİ) Yorum öğesinin altına eklemek için HTML döndür.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Yorum metninden sonra, yorum içeriği öğesi içinde eklemek için HTML döndür (anketler tarafından kullanılır).
 * @property {Function} [replyAreaInputBottom] - Metin girişinin altında, yorum giriş çerçevesi içinde eklemek için HTML döndür (yerinde anket düzenleyicisi için anketler tarafından kullanılır). Üst yorum kimliğini alır, kök yanıt kutusu için null olabilir.
 * @property {Function} [onPollUpdate] - Sayfadaki bir anketin oy sayıları değiştiğinde canlı olayla birlikte çağrılır.
 * @property {Function} isSiteAdmin - Görüntüleyicinin kiracının yöneticisi veya moderatörü olup olmadığını döndürür. İlk getirme işleminden sonra bilinir.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ESKİ) Her yorum için menü öğesinin altına eklemek için HTML döndür.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Kök öğe.
 * @param {Object.<string, Function>} clickListeners - Sınıf adına göre tıklama olay işleyicileri, referansla değiştirilebilir.
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