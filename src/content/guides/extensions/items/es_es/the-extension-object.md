---
El objeto de extensión consiste en la siguiente definición:

<!-- si desea actualizar esto, recuerde actualizar comment-ui-core -->
[inline-code-attrs-start title = 'Extension Object JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * El objeto de extensión FastCommentsUI. Se utiliza para la carga diferida de ciertos componentes. Por ejemplo, el sistema de reseñas no
 * es utilizado por todos los clientes, por lo que solo cargamos esa extensión cuando la necesitamos.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - El nodo raíz DOM del widget.
 * @property {string} [css]
 * @property {Object} config - El objeto de configuración de FastComments.
 * @property {Object} commentsById - Una referencia a un objeto con todos los comentarios por id, que se mantiene actualizado.
 * @property {Object} translations - Una referencia a todas las traducciones.
 * @property {Function} reRenderComment - Una referencia a una función que puede invocarse para volver a renderizar un comentario.
 * @property {Function} removeCommentAndReRender - Una referencia a una función que puede invocarse para eliminar un comentario de la memoria y volver a renderizar la parte correspondiente del DOM.
 * @property {Function} newBroadcastId - Una referencia a una función que puede invocarse para crear un nuevo broadcast id y añadirlo a la lista local de broadcast ids a ignorar.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtrar el HTML para el área de comentarios.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtrar el HTML de todo el widget al renderizar.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtrar el HTML de cada comentario antes de renderizar.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtrar el HTML de cada menú de comentario antes de renderizar.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtrar el HTML de todo el widget al renderizar.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGADO) Devolver HTML para añadir en la parte superior del área de respuesta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGADO) Devolver HTML para añadir en la parte superior del widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGADO) Devolver HTML para añadir en la parte superior del elemento de comentario.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGADO) Devolver HTML para añadir en la parte inferior del elemento de comentario.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGADO) Devolver HTML para añadir en la parte inferior del elemento de menú de cada comentario.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - El elemento raíz.
 * @param {Object.<string, Function>} clickListeners - Los manejadores de eventos para clics, por nombre de clase, que pueden modificarse por referencia.
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


---