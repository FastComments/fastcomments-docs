L'objet d'extension se compose de la définition suivante :

<!-- si vous voulez mettre à jour ceci, souvenez-vous de mettre à jour comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc de l’extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Le nœud DOM racine du widget.
 * @property {string} [css]
 * @property {Object} config - L'objet de configuration FastComments.
 * @property {Object} commentsById - Une référence à un objet contenant tous les commentaires par id, qui est tenu à jour.
 * @property {Object} translations - Une référence à toutes les traductions.
 * @property {Function} reRenderComment - Une référence à une fonction pouvant être appelée pour réafficher un commentaire.
 * @property {Function} removeCommentAndReRender - Une référence à une fonction pouvant être appelée pour supprimer un commentaire de la mémoire et réafficher la partie appropriée du DOM.
 * @property {Function} newBroadcastId - Une référence à une fonction pouvant être appelée pour créer un nouvel identifiant de diffusion (broadcast id) et l'ajouter à la liste locale d'identifiants de diffusion à ignorer.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtre HTML pour la zone de commentaire.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtre HTML pour l'ensemble du widget lors du rendu.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtre HTML pour chaque commentaire avant le rendu.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtre HTML pour le menu de chaque commentaire avant le rendu.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtre HTML pour l'ensemble du widget lors du rendu.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (HÉRITAGE) Retourne du HTML à ajouter en haut de la zone de réponse.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (HÉRITAGE) Retourne du HTML à ajouter en haut du widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (HÉRITAGE) Retourne du HTML à ajouter en haut de l'élément de commentaire.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (HÉRITAGE) Retourne du HTML à ajouter en bas de l'élément de commentaire.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (HÉRITAGE) Retourne du HTML à ajouter en bas de l'élément de menu pour chaque commentaire.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - L'élément racine.
 * @param {Object.<string, Function>} clickListeners - Les gestionnaires d'événements pour les clics, par nom de classe, qui peuvent être modifiés par référence.
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