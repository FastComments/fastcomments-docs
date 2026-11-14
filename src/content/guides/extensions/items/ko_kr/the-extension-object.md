확장(extension) 객체는 다음과 같은 정의로 구성됩니다:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = '확장 객체 JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI 확장 객체입니다. 특정 컴포넌트를 지연 로딩(lazy-loading)하는 데 사용됩니다. 예를 들어 리뷰 시스템은
 * 모든 고객이 사용하는 것이 아니므로, 필요할 때에만 해당 확장을 로드합니다.
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - 위젯의 루트 DOM 노드입니다.
 * @property {string} [css]
 * @property {Object} config - FastComments 설정(config) 객체입니다.
 * @property {Object} commentsById - 모든 댓글을 id별로 담고 있는 객체에 대한 참조이며, 항상 최신 상태로 유지됩니다.
 * @property {Object} translations - 모든 번역에 대한 참조입니다.
 * @property {Function} reRenderComment - 호출하여 댓글을 다시 렌더링할 수 있는 함수에 대한 참조입니다.
 * @property {Function} removeCommentAndReRender - 호출하여 메모리에서 댓글을 제거하고 DOM의 해당 부분을 다시 렌더링할 수 있는 함수에 대한 참조입니다.
 * @property {Function} newBroadcastId - 호출하여 새 broadcast id를 생성하고 무시할 로컬 broadcast id 목록에 추가하는 함수에 대한 참조입니다.
 * @property {Function} [withSessionId] - 요청 URL과 콜백을 인자로 호출합니다. 서드파티 쿠키가 차단된 경우(댓글 작성 및 투표 요청이 사용하는 것과 동일한 경로) 뷰어의 세션 id가 URL에 추가된 상태로 콜백을 실행하고, 그렇지 않으면 URL을 그대로 전달합니다. SSO에는 사용되지 않습니다.
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 게시하려는 댓글과 함께 호출됩니다. false를 반환하면 제출이 취소됩니다(예: 첨부된 투표가 완성되지 않은 경우).
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - 댓글 영역의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - 렌더링 시 위젯 전체의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 렌더링 전에 각 댓글의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 렌더링 전에 각 댓글 메뉴의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - 렌더링 시 위젯 전체의 HTML을 필터링합니다.
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (레거시) 답글 영역 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (레거시) 위젯 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (레거시) 댓글 요소 상단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (레거시) 댓글 요소 하단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - 댓글 텍스트 뒤, 댓글 콘텐츠 요소 내부에 추가할 HTML을 반환합니다(투표에서 사용).
 * @property {Function} [replyAreaInputBottom] - 댓글 입력 프레임 내부, 텍스트 입력 아래에 추가할 HTML을 반환합니다(투표의 인라인 투표 편집기에서 사용). 부모 댓글 id를 전달받으며, 루트 답글 상자의 경우 null이 전달됩니다.
 * @property {Function} [onPollUpdate] - 페이지의 투표 투표 수가 변경될 때 라이브 이벤트와 함께 호출됩니다.
 * @property {Function} isSiteAdmin - 뷰어가 테넌트의 관리자 또는 중재자인지 여부를 반환합니다. 첫 번째 fetch 이후에 알 수 있습니다.
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (레거시) 각 댓글의 메뉴 요소 하단에 추가할 HTML을 반환합니다.
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - 루트 요소입니다.
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