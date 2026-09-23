拡張オブジェクトは以下の定義で構成されています:

<!-- これを更新したい場合は、comment-ui-core を更新することを忘れないでください -->
[inline-code-attrs-start title = '拡張オブジェクト JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI の拡張オブジェクトです。特定のコンポーネントを遅延ロードするために使用されます。例えば、レビューシステムは
 * すべての顧客が使用するわけではないため、必要なときにだけこの拡張をロードします。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - ウィジェットのルート DOM ノード。
 * @property {string} [css]
 * @property {Object} config - FastComments の設定オブジェクト。
 * @property {Object} commentsById - ID 別にすべてのコメントを保持するオブジェクトへの参照で、常に最新の状態が保たれます。
 * @property {Object} translations - すべての翻訳への参照。
 * @property {Function} reRenderComment - コメントを再描画するために呼び出せる関数への参照。
 * @property {Function} removeCommentAndReRender - コメントをメモリから削除し、DOM の該当部分を再描画するために呼び出せる関数への参照。
 * @property {Function} newBroadcastId - 新しいブロードキャスト ID を作成し、無視するローカルリストに追加できる関数への参照。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - 投稿される直前のコメントが渡されます。false を返すと送信がキャンセルされます（例：添付された投票が未完了の場合）。
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - コメントエリアの HTML をフィルタリングします。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - ウィジェット全体のレンダリング時に HTML をフィルタリングします。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 各コメントのレンダリング前に HTML をフィルタリングします。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 各コメントメニューのレンダリング前に HTML をフィルタリングします。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - ウィジェット全体のレンダリング時に HTML をフィルタリングします。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) 返信エリアの上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) ウィジェットの上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) コメント要素の上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) コメント要素の下部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - コメントテキストの後、コメントコンテンツ要素内に追加する HTML を返します（投票で使用）。
 * @property {Function} [replyAreaInputBottom] - テキスト入力の下に、コメント入力フレーム内に追加する HTML を返します（インライン投票エディタで使用）。親コメント ID、またはルート返信ボックスの場合は null が渡されます。
 * @property {Function} [onPollUpdate] - ページ上の投票の票数が変化したときにライブイベントとして呼び出されます。
 * @property {Function} isSiteAdmin - ビューアがテナントの管理者またはモデレーターかどうかを返します。最初の取得後に判明します。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) 各コメントのメニュー要素の下部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - ルート要素。
 * @param {Object.<string, Function>} clickListeners - クラス名ごとのクリックイベントハンドラで、参照によって変更可能です。
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