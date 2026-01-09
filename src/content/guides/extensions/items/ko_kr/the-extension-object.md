확장 객체는 다음 정의로 구성됩니다:

<!-- 이것을 업데이트하려면 comment-ui-core도 업데이트해야 함을 기억하세요 -->
[inline-code-attrs-start title = '확장 객체 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. Used for lazy-loading certain components. For example, the review system is not
 * used by all customers, so we only load that extension when we want it.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 위젯 루트 DOM 노드.
 * @property {string} [css]
 * @property {Object} config - FastComments 구성 객체.
 * @property {Object} commentsById - ID별 모든 댓글을 담고 있고 최신 상태로 유지되는 객체에 대한 참조.
 * @property {Object} translations - 모든 번역에 대한 참조.
 * @property {Function} reRenderComment - 댓글을 다시 렌더링하기 위해 호출할 수 있는 함수에 대한 참조.
 * @property {Function} removeCommentAndReRender - 메모리에서 댓글을 제거하고 DOM의 해당 부분을 다시 렌더링하기 위해 호출할 수 있는 함수에 대한 참조.
 * @property {Function} newBroadcastId - 새 브로드캐스트 ID를 생성하고 무시할 로컬 브로드캐스트 ID 목록에 추가하기 위해 호출할 수 있는 함수에 대한 참조.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 댓글 영역의 HTML을 필터합니다.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 렌더링 시 전체 위젯의 HTML을 필터합니다.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 렌더링 전에 각 댓글의 HTML을 필터합니다.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 각 댓글 메뉴를 렌더링하기 전에 HTML을 필터합니다.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 렌더링 시 전체 위젯의 HTML을 필터합니다.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) 답글 영역 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) 위젯 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) 댓글 요소 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) 댓글 요소 하단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) 각 댓글의 메뉴 요소 하단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 루트 요소.
 * @param {Object.<string, Function>} clickListeners - 클래스 이름별 클릭 이벤트 핸들러로, 참조를 통해 수정할 수 있습니다.
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