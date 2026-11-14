Uzantı (extension) nesnesi aşağıdaki tanımdan oluşur:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Extension Object JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI uzantı nesnesi. Belirli bileşenleri tembel yüklemek (lazy-loading) için kullanılır. Örneğin, yorum
 * (review) sistemi tüm müşteriler tarafından kullanılmadığından, bu uzantıyı yalnızca ihtiyaç duyduğumuzda yükleriz.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Widget'ın kök DOM düğümü.
 * @property {string} [css]
 * @property {Object} config - FastComments yapılandırma (config) nesnesi.
 * @property {Object} commentsById - Tüm yorumları id'ye göre içeren ve güncel tutulan bir nesneye referans.
 * @property {Object} translations - Tüm çevirilere referans.
 * @property {Function} reRenderComment - Bir yorumu yeniden render etmek için çağrılabilen bir fonksiyona referans.
 * @property {Function} removeCommentAndReRender - Bir yorumu bellekten kaldırmak ve DOM'un ilgili bölümünü yeniden render etmek için çağrılabilen bir fonksiyona referans.
 * @property {Function} newBroadcastId - Yeni bir broadcast id oluşturmak ve yok sayılacak yerel broadcast id listesine eklemek için çağrılabilen bir fonksiyona referans.
 * @property {Function} [withSessionId] - Bir istek URL'si ve bir callback ile çağırın. Üçüncü taraf çerezleri engellendiğinde (yorum ve oylama isteklerinin kullandığı aynı yol üzerinden) URL'ye izleyicinin session id'sini ekleyerek callback'i çağırır; aksi halde URL'yi değiştirmeden iletir. Bu yolla bir session id mevcut olmadığında, varsa, rememberAnonSessionId'e en son verilen anonim session id'yi ekler. SSO için kullanılmaz.
 * @property {Function} [rememberAnonSessionId] - Bir yanıtın döndürdüğü anonim session id ile çağırın. Sunucu bunu yalnızca isteğinde session çerezi olmayan bir misafire döndürür ve oturum açmış bir oturum için asla döndürmez. localStorage'da (depolama kullanılamıyorsa bellekte) tutulur; böylece withSessionId sonraki isteklerde bunu gönderir ve misafir tek bir kimlik korur.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Gönderilmek üzere olan yorum ile çağrılır. Gönderimi iptal etmek için false döndürün (örneğin, eklenen anket eksik olduğunda).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Yorum alanı için HTML filtresi.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Render sırasında tüm widget için HTML filtresi.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Render öncesinde her yorum için HTML filtresi.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Render öncesinde her yorum menüsü için HTML filtresi.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Render sırasında tüm widget için HTML filtresi.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (ESKİ/LEGACY) Yanıt alanının üstüne eklenecek HTML döndürün.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (ESKİ/LEGACY) Widget'ın üstüne eklenecek HTML döndürün.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (ESKİ/LEGACY) Yorum öğesinin üstüne eklenecek HTML döndürün.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (ESKİ/LEGACY) Yorum öğesinin altına eklenecek HTML döndürün.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Yorum metninden sonra, yorum içerik öğesinin içinde eklenecek HTML döndürün (anketler tarafından kullanılır).
 * @property {Function} [replyAreaInputBottom] - Yorum giriş çerçevesinin içinde, metin girişinin altına eklenecek HTML döndürün (anketler tarafından yerinde anket düzenleyicisi için kullanılır). Üst yorumun id'sini alır; kök yanıt kutusu için null alır.
 * @property {Function} [onPollUpdate] - Sayfadaki bir anketin oy sayıları değiştiğinde canlı olay ile çağrılır.
 * @property {Function} isSiteAdmin - İzleyicinin kiracının (tenant) yöneticisi veya moderatörü olup olmadığını döndürür. İlk veri çekilişinden sonra bilinir.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (ESKİ/LEGACY) Her yorum için menü öğesinin altına eklenecek HTML döndürün.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - Kök öğe.
 * @param {Object.<string, Function>} clickListeners - Sınıf adına göre tıklama olay işleyicileri; referans yoluyla değiştirilebilir.
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