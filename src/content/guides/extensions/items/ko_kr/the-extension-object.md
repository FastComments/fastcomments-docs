The extension object consists of the following definition:

<!-- 이 것을 업데이트하려면 comment-ui-core를 업데이트하는 것을 기억하세요 -->

[inline-code-attrs-start title = '확장 객체 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 확장 객체입니다. 특정 컴포넌트를 지연 로드하는 데 사용됩니다. 예를 들어, 리뷰 시스템은 모든 고객이 사용하는 것이 아니므로 필요할 때만 해당 확장을 로드합니다.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 위젯 루트 DOM 노드.
 * @property {string} [css]
 * @property {Object} config - FastComments 구성 객체.
 * @property {Object} commentsById - ID별 모든 댓글을 포함하는 객체에 대한 참조이며, 최신 상태를 유지합니다.
 * @property {Object} translations - 모든 번역에 대한 참조.
 * @property {Function} reRenderComment - 댓글을 다시 렌더링할 수 있는 함수에 대한 참조.
 * @property {Function} removeCommentAndReRender - 메모리에서 댓글을 제거하고 DOM의 해당 부분을 다시 렌더링할 수 있는 함수에 대한 참조.
 * @property {Function} newBroadcastId - 새 방송 ID를 생성하고 무시할 로컬 방송 ID 목록에 추가할 수 있는 함수에 대한 참조.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 게시될 댓글과 함께 호출됩니다. 제출을 취소하려면 false를 반환합니다(예: 첨부된 설문이 완전하지 않은 경우).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 댓글 영역의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 렌더링 시 전체 위젯의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 렌더링 전에 각 댓글의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 렌더링 전에 각 댓글 메뉴의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 렌더링 시 전체 위젯의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) 답글 영역 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) 위젯 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) 댓글 요소 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) 댓글 요소 하단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 댓글 텍스트 뒤, 댓글 내용 요소 내부에 추가할 HTML을 반환합니다(설문에 사용).
 * @property {Function} [replyAreaInputBottom] - 텍스트 입력 아래, 댓글 입력 프레임 내부에 추가할 HTML을 반환합니다(인라인 설문 편집기에 사용). 부모 댓글 ID를 받으며, 루트 답글 박스인 경우 null을 받습니다.
 * @property {Function} [onPollUpdate] - 페이지의 설문 투표 수가 변경될 때 실시간 이벤트와 함께 호출됩니다.
 * @property {Function} isSiteAdmin - 뷰어가 테넌트의 관리자 또는 모더레이터인지 여부를 반환합니다. 첫 번째 가져오기 후에 알 수 있습니다.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) 각 댓글의 메뉴 요소 하단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 루트 요소.
 * @param {Object.<string, Function>} clickListeners - 클래스 이름별 클릭 이벤트 핸들러이며, 참조를 통해 수정할 수 있습니다.
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