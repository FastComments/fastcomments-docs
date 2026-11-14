extension オブジェクトは以下の定義で構成されています:

<!-- if you want to update this, remember to update comment-ui-core -->
[inline-code-attrs-start title = 'Extension Object JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI の extension オブジェクト。特定のコンポーネントを遅延読み込みするために使用します。例えば、レビュー
 * システムはすべての顧客が使用するわけではないため、必要なときにのみその extension を読み込みます。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - ウィジェットのルート DOM ノード。
 * @property {string} [css]
 * @property {Object} config - FastComments の設定オブジェクト。
 * @property {Object} commentsById - ID ごとのすべてのコメントを含むオブジェクトへの参照。常に最新の状態に保たれます。
 * @property {Object} translations - すべての翻訳への参照。
 * @property {Function} reRenderComment - 呼び出すことでコメントを再レンダリングできる関数への参照。
 * @property {Function} removeCommentAndReRender - 呼び出すことでコメントをメモリから削除し、DOM の該当部分を再レンダリングできる関数への参照。
 * @property {Function} newBroadcastId - 呼び出すことで新しい broadcast id を作成し、無視する broadcast id のローカルリストに追加できる関数への参照。
 * @property {Function} [withSessionId] - リクエスト URL とコールバックを引数に呼び出します。サードパーティ Cookie がブロックされている場合(コメントおよび投票リクエストと同じ経路)、ビューアーのセッション ID を付加した URL でコールバックを呼び出し、それ以外の場合は URL をそのまま渡します。その方法でセッション ID が取得できない場合は、rememberAnonSessionId に最後に渡された匿名セッション ID があればそれを付加します。SSO では使用されません。
 * @property {Function} [rememberAnonSessionId] - レスポンスが返した anonSessionId を引数に呼び出します。サーバーは、リクエストにセッション Cookie がないゲストに対してのみこれを返し、ログイン済みセッションでは決して返しません。localStorage(ストレージが利用できない場合はメモリ)に保持されるため、withSessionId が以降のリクエストでそれを送信し、ゲストは同一の識別情報を維持できます。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 投稿されようとしているコメントとともに呼び出されます。false を返すと送信をキャンセルします(例:添付された投票が未完了の場合)。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - コメントエリアの HTML をフィルターします。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - レンダリング時にウィジェット全体の HTML をフィルターします。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - レンダリング前に各コメントの HTML をフィルターします。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - レンダリング前に各コメントメニューの HTML をフィルターします。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - レンダリング時にウィジェット全体の HTML をフィルターします。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (レガシー) 返信エリアの上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (レガシー) ウィジェットの上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (レガシー) コメント要素の上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (レガシー) コメント要素の下部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - コメントテキストの後に、コメントコンテンツ要素内に追加する HTML を返します(投票で使用されます)。
 * @property {Function} [replyAreaInputBottom] - コメント入力フレーム内、テキスト入力の下に追加する HTML を返します(投票ではインプレース投票エディターに使用されます)。親コメントの ID を受け取ります。ルートの返信ボックスの場合は null が渡されます。
 * @property {Function} [onPollUpdate] - ページ上の投票の投票数が変化したときに、ライブイベントとともに呼び出されます。
 * @property {Function} isSiteAdmin - ビューアーがテナントの管理者またはモデレーターであるかどうかを返します。最初のフェッチ後に判明します。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (レガシー) 各コメントのメニュー要素の下部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - ルート要素。
 * @param {Object.<string, Function>} clickListeners - クリックのイベントハンドラー。クラス名で管理され、参照経由で変更できます。
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