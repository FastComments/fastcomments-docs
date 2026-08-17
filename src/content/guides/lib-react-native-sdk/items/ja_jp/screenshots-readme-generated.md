Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>ライブコメント</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="ライブコメント、ライトテーマ"/></td>
    <td align="center"><b>ダークテーマ</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="ライブコメント、ダークテーマ"/></td>
    <td align="center"><b>ライブチャット</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="ライブチャットプリセット"/></td>
  </tr>
</table>

### リッチテキストエディタ

このライブラリはリッチテキスト編集のために [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) を使用しており、強力なWYSIWYG編集体験を提供します。同じエディタが iOS、Android、そしてウェブ（`react-native-web` 経由）で動作するため、コンポーザーは単一の実装で全プラットフォームで一貫した動作をします。

`react-native-enriched` はネイティブで React Native New Architecture (Fabric) が必要です（RN 0.76 以降はデフォルト、RN 0.72-0.75 ではオプトイン）。また、パッケージの `exports` 条件を解決できるバンドラが必要です。この SDK は RN 0.81 / React 19 を対象に開発・テストされています。同じエディタは `react-native-web` を通じてウェブでも動作しますが、enriched エディタのウェブビルドはまだ上流で実験的とされています。

### ウィジェット

SDK には FastComments Android SDK を鏡像した 3 つのウィジェットが同梱されています:

- `FastCommentsLiveCommenting` - 投票、返信、ページネーション、メンション、通知、ライブ更新を備えたスレッドコメント。
- `FastCommentsLiveChat` - 同じエンジン上のチャットプリセット：新しいメッセージが下部に表示される時系列メッセージ、リストの下にコンポーザー、ライブヘッダー（接続ドット + ユーザー数）、上方向スクロールで無限に履歴をロード、未読メッセージへの自動スクロール、投票や返信スレッドはなし。すべてのプリセットは `config` で上書き可能です。
- `FastCommentsFeed` - 投稿コンポーザー、メディア、リアクション、フォロー、ライブの新規投稿バナーを備えたソーシャルフィード。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### テーマ設定

デフォルトの外観はセマンティックデザイントークン（`FastCommentsTheme`）のセットから生成されます：色、間隔、半径、フォントサイズ、フォントウェイト、アバターサイズ。任意のウィジェットの `theme` プロップに部分的なトークンオーバーライド（型は `FastCommentsThemeOverrides`）を渡すと、全体のスタイルツリーが一貫して再スタイル化されます:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

ダークモードはトークンセットを1つ切り替えるだけで利用できます:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` プロップは依然として生の `IFastCommentsStyles` ツリーを受け取り、細かい制御が可能です。`theme` と `styles` の両方が提供された場合、明示的なスタイルがテーマツリーより優先されます。`styles` のみが提供された場合、デフォルトを完全に置き換えます（元の動作であり、既存の統合やスキンは影響を受けません）。`setupDarkModeSkin` は `theme` プロップに置き換えられ、非推奨となります。

### 設定オプション

このライブラリは、Web 実装と同様に、[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) で定義されたすべての設定オプションをサポートすることを目指しています。

これらに加えて、React Native は `FastCommentsRNConfig` を通じていくつかの SDK 固有オプションを追加します:

- `hideTopBar` - コンポーザー上部に表示されるログインユーザー／通知ベルのストリップを非表示にします。
- `usePressToEdit` - コメントを長押ししてメニューを開きます。
- `disableDownVoting` - ダウン投票ボタンを非表示にします。
- `renderCommentInline` - コメント内容と同じ HTML ブロック内にコメント投稿者情報を表示します。
- `renderLikesToRight` - 投票/いいね領域をコメントの下ではなく右側に移動します。
- `renderDateBelowComment` - 日付をコメントの下に表示します。
- `showLiveStatus` - コメント上部にチャットスタイルの「Live」＋ユーザー数ヘッダーを表示します。
- `useInlineSubmitButton` - 送信ボタンをコンポーザー内のアイコンとして表示します。
- `countAboveToggle` - `useShowCommentsToggle` と併用し、"Show Comments" トグルの上に表示するコメント数を指定します。
- `preserveFeedScrollPosition` - `FastCommentsFeed` はアンマウント/リマウント間でスクロール位置を保持します（デフォルト true）。

### FastComments の概念

開始する際に把握しておくべき主な概念は `tenantId` と `urlId` です。`tenantId` は FastComments.com のアカウント識別子です。`urlId` はコメントスレッドが紐付く対象です。ページの URL、製品 ID、記事 ID などが該当します。

### ローカリゼーション

これらのウィジェット内のすべてのユーザー向けテキスト（ボタンラベル、プレースホルダー、空状態、"5 分前" のような相対日時、エラーメッセージ等）は **サーバー駆動** です。コンポーネントは英語文字列をハードコードせず、要求されたロケールに対して FastComments が提供する翻訳を表示します。

`locale` を設定してロケールをリクエストします:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

`locale` が設定されていない場合、FastComments はテナントのデフォルト言語を提供します。

**テキストの編集:** 翻訳はこの SDK ではなく、FastComments のダッシュボードで管理されます。文言を変更するには、デフォルトのコピーを上書きするか、言語を追加し、ダッシュボードでアカウントの翻訳を編集してください。変更はウィジェットが自動的に取得し、アプリのリリースは不要です。SDK には英語のフォールバックが含まれていないため、ダッシュボードでキーを空にすると空文字が表示されます。サポートするすべてのロケールでキーを設定したままにしてください。

### ユーザー通知

FastComments は [多数のシナリオ](https://docs.fastcomments.com/guide-notifications.html) に対する通知をサポートします。通知は設定可能で、全体または通知/コメント単位でオプトアウトでき、ページレベルの購読もサポートしているため、ユーザーは特定のページや記事のスレッドを購読できます。

例えば、Secure SSO を使用してユーザーを認証し、定期的に未読通知をポーリングしてユーザーにプッシュすることが可能です。

未読ユーザー通知の取得と翻訳方法については、[例の AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) を参照してください。

### Gif ブラウザ

デフォルトでは画像や GIF の選択は有効になっていません。画像や GIF のアップロードをサポートする方法については [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) を参照してください。このライブラリには検索と画像を匿名化する Gif ブラウザがあり、単にそれを使用すればよいです。

### パフォーマンス

パフォーマンス上の問題を特定した場合は、使用したデバイスを含む再現例とともにチケットを作成してください。パフォーマンスはすべての FastComments ライブラリにおいて重要な要素です。