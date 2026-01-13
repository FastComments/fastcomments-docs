拡張オブジェクトは次の定義で構成されています：

<!-- これを更新する場合は、comment-ui-coreも更新することを忘れないでください -->
[inline-code-attrs-start title = '拡張オブジェクトの JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * The FastCommentsUI extension object. 特定のコンポーネントを遅延読み込みするために使用されます。例えば、レビューシステムは全ての顧客が使用するわけではないため、必要な時にのみその拡張を読み込みます。
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - ウィジェットのルート DOM ノード。
 * @property {string} [css]
 * @property {Object} config - FastComments の設定オブジェクト。
 * @property {Object} commentsById - 全コメントを id ごとに保持するオブジェクトへの参照。常に最新に保たれます。
 * @property {Object} translations - 全翻訳への参照。
 * @property {Function} reRenderComment - コメントを再レンダリングするために呼び出せる関数への参照。
 * @property {Function} removeCommentAndReRender - メモリからコメントを削除し、適切な DOM 部分を再レンダリングするために呼び出せる関数への参照。
 * @property {Function} newBroadcastId - 新しいブロードキャスト ID を作成し、無視するローカルのブロードキャスト ID リストに追加するために呼び出せる関数への参照。
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving]
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - コメント入力エリアの HTML をフィルタします。
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - レンダー時にウィジェット全体の HTML をフィルタします。
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - 各コメントをレンダリングする前にその HTML をフィルタします。
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - 各コメントメニューをレンダリングする前にその HTML をフィルタします。
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - レンダー時にウィジェット全体の HTML をフィルタします。
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - （レガシー）返信エリアの上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - （レガシー）ウィジェットの上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - （レガシー）コメント要素の上部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - （レガシー）コメント要素の下部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - （レガシー）各コメントのメニュー要素の下部に追加する HTML を返します。
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - ルート要素。
 * @param {Object.<string, Function>} clickListeners - クラス名ごとのクリック用イベントハンドラ。参照で変更可能です。
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