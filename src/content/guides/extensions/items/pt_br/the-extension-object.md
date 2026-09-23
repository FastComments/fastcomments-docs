O objeto de extensão consiste na seguinte definição:

<!-- se você quiser atualizar isso, lembre-se de atualizar comment-ui-core -->
[inline-code-attrs-start title = 'Objeto de Extensão JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * O objeto de extensão FastCommentsUI. Usado para carregamento preguiçoso (lazy-loading) de certos componentes. Por exemplo, o sistema de avaliações não é usado por todos os clientes, então carregamos essa extensão apenas quando precisamos dela.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - O nó DOM raiz do widget.
 * @property {string} [css]
 * @property {Object} config - O objeto de configuração do FastComments.
 * @property {Object} commentsById - Uma referência a um objeto com todos os comentários por id, mantido atualizado.
 * @property {Object} translations - Uma referência a todas as traduções.
 * @property {Function} reRenderComment - Uma referência a uma função que pode ser invocada para re-renderizar um comentário.
 * @property {Function} removeCommentAndReRender - Uma referência a uma função que pode ser invocada para remover um comentário da memória e re-renderizar a parte apropriada do DOM.
 * @property {Function} newBroadcastId - Uma referência a uma função que pode ser invocada para criar um novo ID de broadcast e adicioná-lo à lista local de IDs de broadcast a serem ignorados.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Chamado com o comentário que está prestes a ser postado. Retorne false para cancelar o envio (por exemplo, quando uma enquete anexada está incompleta).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtra o HTML para a área de comentário.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtra o HTML para todo o widget na renderização.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtra o HTML para cada comentário antes da renderização.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtra o HTML para cada menu de comentário antes da renderização.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtra o HTML para todo o widget na renderização.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGADO) Retorna HTML a ser adicionado ao topo da área de resposta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGADO) Retorna HTML a ser adicionado ao topo do widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGADO) Retorna HTML a ser adicionado ao topo do elemento de comentário.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGADO) Retorna HTML a ser adicionado ao fundo do elemento de comentário.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Retorna HTML a ser adicionado após o texto do comentário, dentro do elemento de conteúdo do comentário (usado por enquetes).
 * @property {Function} [replyAreaInputBottom] - Retorna HTML a ser adicionado dentro da caixa de entrada de comentário, abaixo da entrada de texto (usado por enquetes para o editor de enquete in-place). Recebe o ID do comentário pai, ou null para a caixa de resposta raiz.
 * @property {Function} [onPollUpdate] - Chamado com o evento ao vivo quando a contagem de votos de uma enquete na página mudar.
 * @property {Function} isSiteAdmin - Retorna se o visualizador é um administrador ou moderador do tenant. Conhecido após a primeira busca.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGADO) Retorna HTML a ser adicionado ao fundo do elemento de menu para cada comentário.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - O elemento raiz.
 * @param {Object.<string, Function>} clickListeners - Os manipuladores de eventos para cliques, por nome de classe, que podem ser modificados por referência.
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