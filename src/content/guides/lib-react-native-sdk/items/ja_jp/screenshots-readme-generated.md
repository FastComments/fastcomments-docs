アバター、ネストされた返信、投票、内蔵のリッチテキストエディターを備えたライブスレッド型コメント機能に加え、ダークテーマとライブチャットのプリセットも提供します(`react-native-web` でのレンダリング例を以下に示します):

<table>
  <tr>
    <td align="center"><b>ライブコメント</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="ライブコメント(ライトテーマ)"/></td>
    <td align="center"><b>ダークテーマ</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="ライブコメント(ダークテーマ)"/></td>
    <td align="center"><b>ライブチャット</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="ライブチャットプリセット"/></td>
  </tr>
</table>

### リッチテキストエディター

このライブラリは、リッチテキスト編集のために [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) を使用しており、強力な WYSIWYG 編集体験を提供します。同じエディターが iOS、Android、そして(`react-native-web` 経由で)ウェブを支えているため、単一の実装でどのプラットフォームでも一貫した動作をするコンポーザーを実現しています。

`react-native-enriched` は、ネイティブ環境では React Native New Architecture(Fabric)(RN 0.76 以降はデフォルト、RN 0.72〜0.75 ではオプトイン)と、パッケージの `exports` 条件を解決できるバンドラーを必要とします。この SDK は RN 0.81 / React 19 に対して開発・テストされています。同じエディターは `react-native-web` を通じてウェブでも動作しますが、enriched エディターのウェブビルドは上流ではまだ実験的とされています。

### ウィジェット

この SDK には、FastComments Android SDK と同様に 3 つのウィジェットが同梱されています:

- `FastCommentsLiveCommenting` - 投票、返信、ページネーション、メンション、通知、ライブ更新を備えたスレッド型コメント機能。
- `FastCommentsLiveChat` - 同じエンジン上のチャットプリセット:新しいメッセージが下部に追加される時系列メッセージ、リストの下にあるコンポーザー、ライブヘッダーストリップ(接続ドット + ユーザー数)、上にスクロールして読み込む無限履歴、新しいメッセージへの自動スクロール、投票や返信スレッドなし。すべてのプリセットは `config` 経由で上書きできます。
- `FastCommentsFeed` - 投稿コンポーザー、メディア、リアクション、フォロー、ライブの新着投稿バナーを備えたソーシャルフィード。

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### テーマ設定

デフォルトの見た目は、一連のセマンティックなデザイントークン(`FastCommentsTheme`)から生成されます:色、スペーシング、角丸、フォントサイズ、フォントウェイト、アバターサイズ。部分的なトークンの上書き(型は `FastCommentsThemeOverrides`)を任意のウィジェットの `theme` プロパティに渡すと、スタイルツリー全体が一貫して再スタイルされます:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

ダークモードもトークンセットを 1 つ渡すだけで切り替えられます:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` プロパティは、きめ細かい制御のために生の `IFastCommentsStyles` ツリーも引き続き受け付けます。`theme` と `styles` の両方が指定された場合は、明示的なスタイルがテーマツリーより優先されます。`styles` のみが指定された場合は、デフォルトを完全に置き換えます(元の動作のため、既存の統合やスキンには影響しません)。`setupDarkModeSkin` は非推奨となり、代わりに `theme` プロパティを使用してください。

### 設定オプション

このライブラリは、ウェブ実装と同様に、[fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) で定義されているすべての設定オプションをサポートすることを目指しています。

さらに、React Native では `FastCommentsRNConfig` 経由でいくつかの SDK 固有のオプションが追加されています:

- `hideTopBar` - コンポーザーの上に表示されるログインユーザー / 通知ベルのストリップを非表示にします。
- `usePressToEdit` - コメントを長押しするとメニューが開きます。
- `disableDownVoting` - ダウン投票ボタンを非表示にします。
- `renderCommentInline` - コメント投稿者の情報を、コメント本文と同じ HTML ブロック内にレンダリングします。
- `renderLikesToRight` - 投票/いいねエリアをコメントの下ではなく右側に移動します。
- `renderDateBelowComment` - 日付をコメントの下にレンダリングします。
- `showLiveStatus` - コメントの上にチャット風の「Live」+ ユーザー数のヘッダーストリップを表示します。
- `useInlineSubmitButton` - 送信ボタンをコンポーザー内のアイコンとしてレンダリングします。
- `countAboveToggle` - `useShowCommentsToggle` 使用時、「Show Comments」トグルの上に表示するコメント数。
- `preserveFeedScrollPosition` - `FastCommentsFeed` がアンマウント/再マウント間でスクロール位置を記憶します(デフォルトは true)。

### FastComments の概念

始めるにあたって把握しておくべき主な概念は `tenantId` と `urlId` です。`tenantId` はあなたの FastComments.com アカウントの識別子です。`urlId` はコメントスレッドが紐付けられる場所です。これはページの URL、商品 ID、記事 ID などが該当します。

### ローカライゼーション

これらのウィジェット内のユーザー向けテキスト(ボタンのラベル、プレースホルダー、空の状態、「5分前」のような相対日付、エラーメッセージなど)はすべて **サーバー駆動** です。コンポーネントは英語の文字列をハードコードしておらず、要求されたロケールに対して FastComments が提供する翻訳をレンダリングします。

ロケールを要求するには、config で `locale` を設定します:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es など
};
```

`locale` が設定されていない場合、FastComments はテナントのデフォルト言語を提供します。

**テキストの編集:** 翻訳はこの SDK ではなく、FastComments ダッシュボードで管理されます。文言を変更したり、デフォルトのコピーを上書きしたり、言語を追加したりするには、ダッシュボードでアカウントの翻訳を編集してください。変更はアプリのリリースなしでウィジェットに自動的に反映されます。この SDK には英語のフォールバックが同梱されていないため、ダッシュボードで空にしたキーは空のままレンダリングされます。サポートするすべてのロケールでキーを埋めたままにしてください。

### ユーザー通知

FastComments は[多くのシナリオ](https://docs.fastcomments.com/guide-notifications.html)での通知をサポートしています。通知は設定可能で、グローバルまたは通知/コメント単位でオプトアウトでき、ページレベルのサブスクリプションもサポートしているため、ユーザーは特定のページや記事のスレッドを購読できます。

例えば、Secure SSO を使用してユーザーを認証し、未読通知を定期的にポーリングしてユーザーにプッシュすることが可能です。

未読のユーザー通知の取得と翻訳の方法については、[AppNotificationSecureSSO の例](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx)を参照してください。

### Gif ブラウザー

デフォルトでは、画像や gif の選択は有効になっていません。画像や gif のアップロードをサポートする方法については、[example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) を参照してください。このライブラリには、検索や提供される画像を匿名化する Gif ブラウザーが用意されており、それを使うだけで済みます。

### パフォーマンス

パフォーマンスの問題を発見した場合は、使用したデバイスを含む再現例を添えてチケットを起票してください。パフォーマンスはすべての FastComments ライブラリにおいて第一級の関心事です。