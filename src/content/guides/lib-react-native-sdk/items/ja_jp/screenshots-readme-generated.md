Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

このライブラリは [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) を使用してリッチテキスト編集を実現しており、強力な WYSIWYG 編集体験を提供します。同じエディタが iOS、Android、そして `react-native-web` 経由の Web でも動作するため、単一の実装で全プラットフォームで一貫したコンポーザーの挙動が得られます。

`react-native-enriched` はネイティブ側で React Native New Architecture（Fabric）を必要とします（RN 0.76 以降はデフォルト、RN 0.72‑0.75 はオプトイン）。また、パッケージの `exports` 条件を解決できるバンドラが必要です。この SDK は RN 0.81 / React 19 を対象に開発・テストされています。同じエディタは `react-native-web` を通して Web でも動作しますが、Web 用ビルドは上流で実験的とマークされています。

### Widgets

SDK には FastComments Android SDK に対応する 3 つのウィジェットが同梱されています。

- `FastCommentsLiveCommenting` – 投票、返信、ページング、メンション、通知、ライブ更新を備えたスレッドコメント。
- `FastCommentsLiveChat` – 同じエンジン上のチャットプリセット。下部に新しいメッセージが並び、コンポーザーはリストの下に配置、接続ドットとユーザー数を示すライブヘッダー、スクロールアップで読み込む無限履歴、新着メッセージへの自動スクロール、投票やスレッド返信はなし。すべてのプリセットは `config` で上書き可能です。
- `FastCommentsFeed` – 投稿コンポーザー、メディア、リアクション、フォロー、ライブ新規投稿バナーを備えたソーシャルフィード。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

デフォルトの外観は一連のセマンティックデザイントークン（`FastCommentsTheme`）から生成されます：色、間隔、半径、フォントサイズ、フォントウェイト、アバターサイズ。任意のウィジェットの `theme` プロパティに部分的なトークンオーバーライド（型 `FastCommentsThemeOverrides`）を渡すだけで、全スタイルツリーが一貫して再スタイリングされます。

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

ダークモードはトークンセットを 1 つ差し替えるだけです。

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` プロパティは依然として生の `IFastCommentsStyles` ツリーを受け取り、細かい制御が可能です。`theme` と `styles` の両方が提供された場合、明示的な `styles` がテーマツリー上に優先されます。`styles` のみが提供された場合はデフォルト全体が置き換えられます（元の挙動のままで、既存の統合やスキンには影響しません）。`setupDarkModeSkin` は `theme` プロパティに置き換えられ、非推奨となりました。

### Configuration Options

このライブラリは、[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) で定義されたすべての設定オプションをサポートすることを目指しています。Web 実装と同様です。

これに加えて、React Native 固有の SDK オプションが `FastCommentsRNConfig` として提供されます。

- `hideTopBar` – コンポーザー上部に表示されるログインユーザー／通知ベルのストリップを非表示にします。
- `usePressToEdit` – コメントを長押しするとメニューを開きます。
- `disableDownVoting` – ダウン投票ボタンを非表示にします。
- `renderCommentInline` – コメントコンテンツと同じ HTML ブロック内にコメント投稿者情報を表示します。
- `renderLikesToRight` – 投票／いいね領域をコメントの下ではなく右側に配置します。
- `renderDateBelowComment` – 日付をコメントの下に表示します。
- `showLiveStatus` – コメント上部にチャットスタイルの「Live」＋ユーザー数ヘッダーを表示します。
- `useInlineSubmitButton` – コンポーザー内にアイコンとして送信ボタンを表示します。
- `countAboveToggle` – `useShowCommentsToggle` と組み合わせた際に、「Show Comments」トグルの上に表示するコメント数を指定します。
- `preserveFeedScrollPosition` – `FastCommentsFeed` はアンマウント/リマウント間でスクロール位置を保持します（デフォルト true）。

### FastComments Concepts

開始時に把握すべき主要概念は `tenantId` と `urlId` です。`tenantId` は FastComments.com アカウントの識別子、`urlId` はコメントスレッドが紐付く対象です。ページ URL、商品 ID、記事 ID など、任意の文字列を指定できます。

### Localization

これらウィジェットのユーザー向けテキスト（ボタンラベル、プレースホルダー、空状態、"5 minutes ago" などの相対日時、エラーメッセージ等）は **サーバードリブン** です。コンポーネントは英語文字列をハードコードせず、要求されたロケールに対して FastComments が提供する翻訳をそのまま表示します。

ロケールを指定するには、設定に `locale` を設定します。

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

`locale` が設定されていない場合、FastComments はテナントのデフォルト言語を使用します。

**テキストの編集:** 翻訳は FastComments ダッシュボード上で管理され、SDK 内では行いません。文言を変更したい場合は、デフォルトのコピーを上書きするか、言語を追加し、ダッシュボードで翻訳を編集してください。ウィジェットは自動的に変更を取得し、アプリのリリースは不要です。SDK には英語のフォールバックが含まれていないため、ダッシュボードでキーを空にすると空文字が表示されます。サポートするすべてのロケールでキーは埋めておくようにしてください。

### User Notifications

FastComments は多数のシナリオに対する通知をサポートしています（[こちら](https://docs.fastcomments.com/guide-notifications.html)）。通知は設定可能で、全体的にオプトアウトできるほか、個別の通知／コメントレベルでも制御できます。また、ページ単位の購読が可能で、ユーザーは特定のページや記事のスレッドに購読できます。

たとえば、Secure SSO を使用してユーザーを認証し、定期的に未読通知をポーリングしてユーザーにプッシュすることができます。

未読ユーザー通知の取得と翻訳方法については、[AppNotificationSecureSSO のサンプル](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) を参照してください。

### Gif Browser

デフォルトでは画像や GIF の選択は無効です。画像・GIF アップロードをサポートする方法は [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) をご覧ください。このライブラリには検索と画像を匿名化する GIF ブラウザが含まれており、必要に応じて利用できます。

### Performance

パフォーマンスに関する問題を特定した場合は、デバイス情報などを含む再現手順とともにチケットを作成してください。パフォーマンスはすべての FastComments ライブラリにおいてファーストクラスの要素です。