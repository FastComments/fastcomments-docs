O objeto de extensão consiste na seguinte definição:

<!-- se você quiser atualizar isto, lembre-se de atualizar comment-ui-core -->
[inline-code-attrs-start title = 'JSDoc do Objeto de Extensão'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - O nó raiz do DOM do widget.
 * @property {string} [css]
 * @property {Object} config - O objeto de configuração do FastComments.
 * @property {Object} commentsById - Uma referência a um objeto com todos os comentários por id, que é mantida atualizada.
 * @property {Object} translations - Uma referência para todas as traduções.
 * @property {Function} reRenderComment - Uma referência para uma função que pode ser invocada para re-renderizar um comentário.
 * @property {Function} removeCommentAndReRender - Uma referência para uma função que pode ser invocada para remover um comentário da memória e re-renderizar a parte apropriada do DOM.
 * @property {Function} newBroadcastId - Uma referência para uma função que pode ser invocada para criar um novo broadcast id e adicioná-lo à lista local de broadcast ids a serem ignorados.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrar o HTML da área de resposta.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrar o HTML de todo o widget ao renderizar.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrar o HTML de cada comentário antes de renderizar.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrar o HTML de cada menu de comentário antes de renderizar.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrar o HTML de todo o widget ao renderizar.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGADO) Retorna HTML para adicionar ao topo da área de resposta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGADO) Retorna HTML para adicionar ao topo do widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGADO) Retorna HTML para adicionar ao topo do elemento de comentário.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGADO) Retorna HTML para adicionar ao final do elemento de comentário.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGADO) Retorna HTML para adicionar ao final do elemento de menu para cada comentário.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - O elemento raiz.
 * @param {Object.<string, Function>} clickListeners - Os manipuladores de evento para cliques, por nome de classe, que podem ser modificados por referência.
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