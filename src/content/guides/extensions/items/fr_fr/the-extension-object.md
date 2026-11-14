L'objet extension est constitué de la définition suivante :

<!-- si vous souhaitez le mettre à jour, pensez à mettre à jour comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc de l\'objet Extension'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * L'objet extension de FastCommentsUI. Utilisé pour le chargement différé de certains composants. Par exemple, le système d'avis n'est pas
 * utilisé par tous les clients, nous ne chargeons donc cette extension que lorsque nous en avons besoin.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - Le nœud DOM racine du widget.
 * @property {string} [css]
 * @property {Object} config - L'objet de configuration FastComments.
 * @property {Object} commentsById - Une référence vers un objet contenant tous les commentaires par id, qui est maintenu à jour.
 * @property {Object} translations - Une référence vers toutes les traductions.
 * @property {Function} reRenderComment - Une référence vers une fonction qui peut être invoquée pour re-rendre un commentaire.
 * @property {Function} removeCommentAndReRender - Une référence vers une fonction qui peut être invoquée pour supprimer un commentaire de la mémoire et re-rendre la partie appropriée du DOM.
 * @property {Function} newBroadcastId - Une référence vers une fonction qui peut être invoquée pour créer un nouvel identifiant de diffusion et l'ajouter à la liste locale des identifiants de diffusion à ignorer.
 * @property {Function} [withSessionId] - Appelé avec une URL de requête et un callback. Invoque le callback avec l'URL plus l'identifiant de session du visiteur ajouté lorsque les cookies tiers sont bloqués (même chemin que les requêtes de commentaire et de vote), ou l'URL inchangée sinon. Non utilisé pour le SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Appelé avec le commentaire sur le point d'être publié. Retourner false pour annuler l'envoi (par exemple lorsqu'un sondage joint est incomplet).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtre le HTML de la zone de commentaires.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtre le HTML de l'ensemble du widget lors du rendu.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtre le HTML de chaque commentaire avant le rendu.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtre le HTML du menu de chaque commentaire avant le rendu.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtre le HTML de l'ensemble du widget lors du rendu.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) Retourne le HTML à ajouter en haut de la zone de réponse.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) Retourne le HTML à ajouter en haut du widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) Retourne le HTML à ajouter en haut de l'élément de commentaire.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) Retourne le HTML à ajouter en bas de l'élément de commentaire.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Retourne le HTML à ajouter après le texte du commentaire, à l'intérieur de l'élément de contenu du commentaire (utilisé par les sondages).
 * @property {Function} [replyAreaInputBottom] - Retourne le HTML à ajouter à l'intérieur du cadre de saisie du commentaire, sous le champ de texte (utilisé par les sondages pour l'éditeur de sondage intégré). Reçoit l'identifiant du commentaire parent, ou null pour la zone de réponse racine.
 * @property {Function} [onPollUpdate] - Appelé avec l'événement en direct lorsque les nombres de votes d'un sondage sur la page changent.
 * @property {Function} isSiteAdmin - Indique si le visiteur est un administrateur ou un modérateur du tenant. Connu après la première récupération.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) Retourne le HTML à ajouter en bas de l'élément de menu de chaque commentaire.
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