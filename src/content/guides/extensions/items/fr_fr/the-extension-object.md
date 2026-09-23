L'objet d'extension se compose de la définition suivante :

<!-- si vous voulez mettre à jour ceci, n'oubliez pas de mettre à jour comment-ui-core -->
[inline-code-attrs-start title = 'Objet d\'extension JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * L'objet d'extension FastCommentsUI. Utilisé pour le chargement différé de certains composants. Par exemple, le système d'évaluation n'est pas utilisé par tous les clients, nous ne chargeons donc cette extension que lorsque nous en avons besoin.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Le nœud DOM racine du widget.
 * @property {string} [css]
 * @property {Object} config - L'objet de configuration FastComments.
 * @property {Object} commentsById - Une référence à un objet contenant tous les commentaires par id, maintenu à jour.
 * @property {Object} translations - Une référence à toutes les traductions.
 * @property {Function} reRenderComment - Une référence à une fonction pouvant être invoquée pour re‑rendre un commentaire.
 * @property {Function} removeCommentAndReRender - Une référence à une fonction pouvant être invoquée pour supprimer un commentaire de la mémoire et re‑rendre la partie appropriée du DOM.
 * @property {Function} newBroadcastId - Une référence à une fonction pouvant être invoquée pour créer un nouvel ID de diffusion et l'ajouter à la liste locale des IDs de diffusion à ignorer.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Appelé avec le commentaire sur le point d'être publié. Retourne false pour annuler la soumission (par exemple lorsqu'un sondage joint est incomplet).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrer le HTML pour la zone de commentaire.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrer le HTML pour l'ensemble du widget lors du rendu.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrer le HTML pour chaque commentaire avant le rendu.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrer le HTML pour chaque menu de commentaire avant le rendu.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrer le HTML pour l'ensemble du widget lors du rendu.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (HÉRITAGE) Retourner du HTML à ajouter en haut de la zone de réponse.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (HÉRITAGE) Retourner du HTML à ajouter en haut du widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (HÉRITAGE) Retourner du HTML à ajouter en haut de l'élément de commentaire.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (HÉRITAGE) Retourner du HTML à ajouter en bas de l'élément de commentaire.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Retourner du HTML à ajouter après le texte du commentaire, à l'intérieur de l'élément de contenu du commentaire (utilisé par les sondages).
 * @property {Function} [replyAreaInputBottom] - Retourner du HTML à ajouter à l'intérieur du cadre de saisie du commentaire, sous le champ texte (utilisé par les sondages pour l'éditeur de sondage intégré). Reçoit l'ID du commentaire parent, ou null pour la boîte de réponse racine.
 * @property {Function} [onPollUpdate] - Appelé avec l'événement en direct lorsque le nombre de votes d'un sondage sur la page change.
 * @property {Function} isSiteAdmin - Renvoie si le visiteur est un administrateur ou modérateur du locataire. Connu après le premier fetch.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (HÉRITAGE) Retourner du HTML à ajouter en bas de l'élément de menu pour chaque commentaire.
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