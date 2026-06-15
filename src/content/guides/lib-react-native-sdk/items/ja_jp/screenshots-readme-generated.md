Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>ライブコメント</b><br/><img src="./demo-screenshots/light.png" width="260" alt="ライブコメント、ライトテーマ"/></td>
    <td align="center"><b>ダークテーマ</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="ライブコメント、ダークテーマ"/></td>
    <td align="center"><b>ライブチャット</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="ライブチャットのプリセット"/></td>
  </tr>
</table>

### Rich Text Editor

このライブラリはリッチテキスト編集に [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) を使用しており、強力な WYSIWYG 編集体験を提供します。同じエディタが iOS、Android、および web（`react-native-web` 経由）で動作するため、コンポーザーは単一の実装でプラットフォーム間で一貫して動作します。

`react-native-enriched` はネイティブ上で React Native の New Architecture (Fabric) を必要とします（RN 0.76 以降はデフォルト、RN 0.72-0.75 ではオプトイン）。また、パッケージの `exports` 条件を解決するバンドラが必要です。本 SDK は RN 0.81 / React 19 を対象に開発およびテストされています。同じエディタは `react-native-web` を通じて web 上でも動作しますが、enriched エディタの web ビルドは上流でまだ実験的とマークされています。

### Widgets

SDK は FastComments Android SDK を反映した 3 つのウィジェットを提供します:

- `FastCommentsLiveCommenting` - 投票、返信、ページネーション、メンション、通知、およびライブ更新を備えたスレッド化コメント。
- `FastCommentsLiveChat` - 同じエンジン上のチャット用プリセット: 新しいメッセージが下に表示される時系列メッセージ、リストの下にコンポーザー、ライブヘッダーストリップ（接続ドット + ユーザー数）、上にスクロールして読み込む無限履歴、新しいメッセージへの自動スクロール、投票や返信スレッドはありません。すべてのプリセットは `config` を介してオーバーライドできます。
- `FastCommentsFeed` - 投稿コンポーザー、メディア、リアクション、フォロー、およびライブ新規投稿バナーを備えたソーシャルフィード。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

デフォルトの外観はセマンティックなデザイントークン群（`FastCommentsTheme`）から生成されます：色、スペーシング、角丸、フォントサイズ、フォントウェイト、アバターサイズ。`theme` プロップに部分的なトークン上書き（型付き `FastCommentsThemeOverrides`）を渡すと、ウィジェット全体のスタイルツリーが一貫して再スタイリングされます：

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

ダークモードはトークンセットを切り替えるだけです：

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` プロップは生の `IFastCommentsStyles` ツリーを受け取って詳細な制御を可能にします。`theme` と `styles` の両方が提供された場合、明示的な `styles` がテーマ化されたツリーより優先されます。`styles` のみが提供された場合はデフォルトを完全に置き換えます（これは従来の挙動であり、既存の統合やスキンに影響を与えません）。`setupDarkModeSkin` は `theme` プロップに置き換えられ、非推奨です。

### Configuration Options

このライブラリは、web 実装と同様に [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) に定義されたすべての構成オプションをサポートすることを目指しています。

それに加えて、React Native は `FastCommentsRNConfig` を通じていくつかの SDK 固有オプションを追加します:

- `hideTopBar` - コンポーザーの上に表示されるログインユーザー / 通知ベルのストリップを非表示にします。
- `usePressToEdit` - コメントを長押ししてメニューを開きます。
- `disableDownVoting` - ダウンボートボタンを非表示にします。
- `renderCommentInline` - コメント内容と同じ HTML ブロック内にコメント投稿者情報をレンダリングします。
- `renderLikesToRight` - 投票/いいね領域をコメントの下ではなく右側に移動します。
- `renderDateBelowComment` - 日付をコメントの下に表示します。
- `showLiveStatus` - コメントの上にチャット風の「Live」+ ユーザー数のヘッダーストリップを表示します。
- `useInlineSubmitButton` - コンポーザー内のアイコンとして送信ボタンをレンダリングします。
- `countAboveToggle` - `useShowCommentsToggle` 使用時に、「Show Comments」トグルの上に表示するコメント数。
- `preserveFeedScrollPosition` - `FastCommentsFeed` がアンマウント/リマウント間でスクロールオフセットを記憶します（デフォルト true）。

### FastComments Concepts

開始するにあたって注目すべき主な概念は `tenantId` と `urlId` です。`tenantId` はあなたの FastComments.com アカウント識別子です。`urlId` はコメントスレッドが紐づく場所です。これはページの URL、製品 ID、記事 ID などになり得ます。

### User Notifications

FastComments は [多くのシナリオ](https://docs.fastcomments.com/guide-notifications.html) に対する通知をサポートします。通知は設定可能で、グローバルまたは通知/コメントレベルでオプトアウトでき、ページレベルの購読をサポートしているため、ユーザーは特定のページや記事のスレッドを購読できます。

たとえば、Secure SSO を使用してユーザーを認証し、その後定期的に未読通知をポーリングしてユーザーにプッシュすることが可能です。

未読ユーザー通知を取得して変換する方法については、[example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) を参照してください。

### Gif Browser

デフォルトでは、画像や GIF の選択は有効になっていません。画像および GIF アップロードをサポートする方法については、[example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) を参照してください。このライブラリには検索と画像を匿名化する Gif Browser が含まれており、単にそれを使用するだけで利用できます。

### Performance

パフォーマンスの問題を特定した場合は、使用デバイスを含む再現可能な例とともにチケットを開いてください。パフォーマンスはすべての FastComments ライブラリにおいて重要な第一級の関心事です。