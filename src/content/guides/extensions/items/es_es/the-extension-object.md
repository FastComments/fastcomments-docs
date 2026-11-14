El objeto de extensión consta de la siguiente definición:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Objeto de extensión JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * El objeto de extensión de FastCommentsUI. Se usa para la carga diferida de ciertos componentes. Por ejemplo, el sistema de reseñas no es
 * usado por todos los clientes, por lo que solo cargamos esa extensión cuando la queremos.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - El nodo dom raíz del widget.
 * @property {string} [css]
 * @property {Object} config - El objeto de configuración de FastComments.
 * @property {Object} commentsById - Una referencia a un objeto con todos los comentarios por id, que se mantiene actualizado.
 * @property {Object} translations - Una referencia a todas las traducciones.
 * @property {Function} reRenderComment - Una referencia a una función que puede invocarse para volver a renderizar un comentario.
 * @property {Function} removeCommentAndReRender - Una referencia a una función que puede invocarse para eliminar un comentario de la memoria y volver a renderizar la parte correspondiente del DOM.
 * @property {Function} newBroadcastId - Una referencia a una función que puede invocarse para crear un nuevo id de difusión y añadirlo a la lista local de ids de difusión a ignorar.
 * @property {Function} [withSessionId] - Se llama con una URL de solicitud y un callback. Invoca el callback con la URL más el id de sesión del espectador añadido cuando las cookies de terceros están bloqueadas (la misma ruta que usan las solicitudes de comentario y voto), o la URL sin cambios en caso contrario. No se usa para SSO.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - Se llama con el comentario a punto de publicarse. Devuelve false para cancelar el envío (por ejemplo, cuando una encuesta adjunta está incompleta).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - Filtra el HTML del área de comentarios.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - Filtra el HTML de todo el widget al renderizar.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - Filtra el HTML de cada comentario antes de renderizar.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - Filtra el HTML del menú de cada comentario antes de renderizar.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - Filtra el HTML de todo el widget al renderizar.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (OBSOLETO) Devuelve HTML para añadir en la parte superior del área de respuesta.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (OBSOLETO) Devuelve HTML para añadir en la parte superior del widget.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (OBSOLETO) Devuelve HTML para añadir en la parte superior del elemento del comentario.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (OBSOLETO) Devuelve HTML para añadir en la parte inferior del elemento del comentario.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - Devuelve HTML para añadir después del texto del comentario, dentro del elemento de contenido del comentario (usado por las encuestas).
 * @property {Function} [replyAreaInputBottom] - Devuelve HTML para añadir dentro del marco de entrada del comentario, debajo del campo de texto (usado por las encuestas para el editor de encuestas en el lugar). Recibe el id del comentario padre, o null para el cuadro de respuesta raíz.
 * @property {Function} [onPollUpdate] - Se llama con el evento en vivo cuando los recuentos de votos de una encuesta en la página cambian.
 * @property {Function} isSiteAdmin - Devuelve si el espectador es administrador o moderador del tenant. Se conoce después de la primera obtención de datos.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (OBSOLETO) Devuelve HTML para añadir en la parte inferior del elemento del menú de cada comentario.
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