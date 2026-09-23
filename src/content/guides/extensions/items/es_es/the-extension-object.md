El objeto de extensión consta de la siguiente definición:

<!-- si deseas actualizar esto, recuerda actualizar comment-ui-core -->
[inline-code-attrs-start title = 'Objeto de Extensión JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * El objeto de extensión FastCommentsUI. Utilizado para cargar perezosamente ciertos componentes. Por ejemplo, el sistema de reseñas no
 * es usado por todos los clientes, por lo que solo cargamos esa extensión cuando la necesitamos.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - El nodo DOM raíz del widget.
 * @property {string} [css]
 * @property {Object} config - El objeto de configuración de FastComments.
 * @property {Object} commentsById - Una referencia a un objeto con todos los comentarios por id, que se mantiene actualizado.
 * @property {Object} translations - Una referencia a todas las traducciones.
 * @property {Function} reRenderComment - Una referencia a una función que puede invocarse para volver a renderizar un comentario.
 * @property {Function} removeCommentAndReRender - Una referencia a una función que puede invocarse para eliminar un comentario de la memoria y volver a renderizar la parte apropiada del DOM.
 * @property {Function} newBroadcastId - Una referencia a una función que puede invocarse para crear un nuevo id de difusión y añadirlo a la lista local de ids de difusión a ignorar.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Se llama con el comentario que está a punto de publicarse. Devuelve false para cancelar el envío (por ejemplo, cuando una encuesta adjunta está incompleta).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtra HTML para el área de comentarios.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtra HTML para todo el widget al renderizar.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtra HTML para cada comentario antes de renderizar.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtra HTML para cada menú de comentario antes de renderizar.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtra HTML para todo el widget al renderizar.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGADO) Devuelve HTML para añadir a la parte superior del área de respuesta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGADO) Devuelve HTML para añadir a la parte superior del widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGADO) Devuelve HTML para añadir a la parte superior del elemento de comentario.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGADO) Devuelve HTML para añadir a la parte inferior del elemento de comentario.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Devuelve HTML para añadir después del texto del comentario, dentro del elemento de contenido del comentario (usado por encuestas).
 * @property {Function} [replyAreaInputBottom] - Devuelve HTML para añadir dentro del marco de entrada del comentario, bajo la entrada de texto (usado por encuestas para el editor de encuestas in situ). Recibe el id del comentario padre, o null para el cuadro de respuesta raíz.
 * @property {Function} [onPollUpdate] - Se llama con el evento en vivo cuando cambian los recuentos de votos de una encuesta en la página.
 * @property {Function} isSiteAdmin - Devuelve si el visor es un administrador o moderador del inquilino. Conocido después de la primera obtención.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGADO) Devuelve HTML para añadir a la parte inferior del elemento de menú para cada comentario.
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